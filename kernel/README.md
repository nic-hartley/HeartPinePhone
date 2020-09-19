# kernel

This is the kernel that actually runs on your hardware.
I have some very specific thoughts on how certain parts of it will work, but please note I'm nowhere _near_ implementing...
Well, most of it, yet.
So the notes below are just that: Notes.
Don't expect them to be used directly or at all.

(P.S.: Expect blogposts on these notes. Y'know, eventually. Probably once I've thought through them more.)

## Scheduler

```c
///////////////////////////////////////////////////////////////////////////////
// Describes the status of a task.
//   This is stored as an unsigned 8-bit field in struct task, so values can
// range from 0-255. The specific values were generally chosen so related
// statuses (e.g. RUNNING and KILLED, which should never be scheduled) share
// bits unique to them. This effectively gives some bits meanings.
//   TODO: Settle on the actual meanings of bits and formalize them.
//   There's a 16-bit status_info field whose meaning depends on the current
// status, and so is described here as well.
enum task_status {
  // Task is completely ready to run, no caveats.
  READY = 0x00,
  // Task is sleeping.
  //   status_info is the number of seconds it should sleep. Because a
  // sleeping task won't be run, last_run won't be updated, so the scheduler
  // can check if last_run + status_info seconds has passed, and if so, set
  // the status to READY.
  //   The task may also be forcibly woken up; in that case, the status will
  // be switched to READY externally.
  //   If this task's sleep time has expired, the scheduler will change its
  // status to READY.
  SLEEPING = 0x01,
  // Task is waiting for the kernel to do something.
  //   status_info is an index into the table of blockers, so the scheduler
  // can "steal" the timeslice to work on the blocker, if the task would
  // otherwise be set to run.
  BLOCKED_KERNEL = 0x02,
  // Task is waiting for another task to do something.
  //   status_info is the task_id of the task it's waiting on. If this task
  // would have been run, the blocking task will be found and, if possible,
  // run instead, taking into account the blocking task's status.
  //   This is used if, for example, the blocking task is holding a mutex
  // this task wants to acquire.
  BLOCKED_TASK = 0x03.
  // Task is currently running (so don't run it here, too).
  //   status_info will contain the index of the core running this, so that
  // the task can be killed.
  RUNNING = 0x04,
  // Task has been killed.
  //   status_info describes why it was killed (e.g. because it was found to
  // be deadlocking).
  KILLED = 0x05.
};

// Describes a single schedulable unit of processing.
//   For untested but napkin-sensible reasons, this is sized to fit four
// instances into a single Cortex-A53 cache line.
//   TL;DR: L1 cache + SIMD = happy scanning.
struct task {
  // A system-unique ID for this task.
  //   Note that to avoid having to sort the task table, this _will not_ line
  // up with the index.
  //   This is set to zero in empty table slots; the rest of the data is left
  // as-is. That allows us to atomically delete a task.
  uint16_t task_id; // 0x00
  // The program that owns this task.
  //   Tasks are individual execution units that can be scheduled. A program
  // could own several by spawning threads, either explicitly or implicitly
  // (e.g. with certain syscalls).
  uint16_t program_id; // 0x02
  // How important this task is.
  //   This is used by the scheduler in conjunction with last_run to pick a
  // task to begin running.
  uint8_t priority; // 0x04
  // Describes the current status of this task.
  //   See the definition of enum task_status for valid values and meanings.
  uint8_t status; // 0x05
  // Used in some statuses to carry more info (e.g. BLOCKED on what).
  //   Meaning depends on status. See enumn task_status for more information.
  //   There may also be more info
  uint16_t status_info; // 0x06
  // The saved stack pointer.
  //   The return info is stored just below the the stack, as though pushed
  // without incrementing the stack pointer.
  void* stack; // 0x08
  // 0x10
};

// It fits neatly into a 64-byte Cortex-A53 L1 cache line.
static_assert(64 % sizeof(struct task) == 0);

#define NUM_TABLES ...
struct task table[NUM_TABLES];

/*
Sketch of scheduling system:

- Tasks are individual "units of execution", like a thread, within a program.
  - Different name because the process/thread system works a bit differently.
    - Process is the primary unit of scheduling on Linux; threads are sort of
      process-lites.
    - Task is the primary unit of scheduling on Heart; a program is just a
      bundle of meta-information that applies to multiple tasks.
- Timer is set up to trigger EL1 interrupt every [n] microseconds
  ? EL1? EL2? Might be useful to have EL1 be just kernel helper code, with EL2
    for stuff like the scheduler.
  ? Function to "lock core exclusively" that masks the interrupt on this core?
  - Syscalls (e.g. sleep) can also jump here, but will set the task's status
  - The other timer is for our clock (probably microsecond resolution? idk)
  - Timer interrupt saves registers, PC onto stack, etc.
    ? Is this necessary?
      How does PSTATE saving work?
- Kernel has some initial, low-priority idle tasks.
  - ID = 0xFFFF to indicate; known indices to distinguish between them
    ? Maybe not specific ID, maybe just special ones? < 16?
  - Otherwise normal tasks with normal priorities
  - Things like parsing backlog of ethernet/USB info, WiFi maintenance, compact
    task table
    ? The task table should be compacted eventually: how to do it safely?
  - Run at EL1 instead of EL0 like apps
? How do I tell which task I just came from?
  ? Get current core, index into something?
? How exactly does that interrupt work?
  Do I need to worry about storing info?
- Set the task we interrupted's last_run to now
- Start iterating through tasks
  ? 4 at a time? SIMD?
  ? PRFM next task?
  - If task status is RUNNING or KILLED, skip this task.
    ? Clean up KILLED tasks on discovery?
  - If task status is SLEEPING, check if last_run + [status_info] seconds is
    after now
    - If so, set status to READY + keep going.
      If not, skip this task.
  - Check if priority * (now - last_run) is largest so far
    ? Additional scaling? Maybe limit max value somehow?
    - If so, store index and value
- If max is less than a specific threshold, make the core idle.
  ? How do we un-idle it? Will the timer interrupt do it?
  ? Tune threshold between power consumption and responsiveness
  ? Is there a power cost to going into idle mode vs. spinning for that time?
- While task at index is BLOCKED_TASK, find its blocker and look at that
  instead.
  - Effectively elevates low-priority task to high-priority if a high-priority
    task is blocked on it.
  ? How to prevent deadlocks using kernel-based synchronization primitives from
    making the scheduler spinlock?
    - Dumb option: Limit to 64 iters, store each task ID; if we come across a
      repeat, kill all the tasks?
- Schedule the task we settle on after the above loop
  - mov sp, task.stack
  - move registers from stack back into, well, registers
    - no loops here, that would use registers, which we're about to clobber
  ? How does returning from an exception work?
    Can't just jump, that'd preserve the exception level.
*/
```


