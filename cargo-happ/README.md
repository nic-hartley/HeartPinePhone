# cargo-happ

A custom cargo command to build a HeartPinePhone app.
It has the same syntax as `cargo build`, plus an extra flag to define flags.
Currently, seeing as the kernel is a Hello World, I've only thought a bit about app formats, and I haven't even begun to actually build this subproject.
If you're curious, here are my notes:

- Apps can be AArch32 or AArch64
- I'd like _every_ section to be relocatable, but I have no idea how to do that
- I really _do not_ want to have to parse ELF.
  That seems hard.
- Maybe a subset of ELF?
- Executables need "requested permissions" embedded somehow into the exe.
  Another ELF section?
- https://wiki.osdev.org/ELF
