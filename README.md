# HeartPinePhone

*aka **Heart** or **HeartPhone**, where unambiguous*

The HeartPinePhone project is a custom kernel for PinePhones.
It focuses hard on security, at the expense of compatibility and even occasionally usability.
If you're looking for a smartphone which prioritizes security and privacy over all else, Heart is for you.
If you want to know more about what security enforcements and features are available, see **§Security**.

While Heart was built to be relatively simple to port to other devices, only PinePhone is truly supported.
In addition, only the latest PinePhone revision (currently, 1.2) is exhaustively tested with every release.
While older ones should work, you may run into bugs.
See **§Bugs** for more information.

The name was chosen because heart pine is the densest type of pine with a simple, English name.
You could say it's like pine, but hardened.
Also, the filename extension derived from it makes me happ-y.

## Writing an app

Want to write your own app for Heart?
Check out [the happ README].

## Vulnerability Disclosure

Have you found what seems like a security issue?
Please report it ASAP to [nic@cybers.eco][0].
I use [ProtonMail][1] to receive email at that address, so if you do too, it'll be encrypted end-to-end.
You can also encrypt with OpenPGP against [my public key][2] if you want.

Please be as specific as possible in your report.
The more detail you provide, the more quickly I'll be able to diagnose and fix the issue.
Ideally, provide the commented source code for a proof-of-concept exploit.
However, this *is not necessary* -- you can report issues even if you can't find a way to exploit them.

If you plan to run any sort of automatic testing against Heart, please let me know.
I might be able to help you run it more efficiently.

Please be aware that this is a side project, and I do have a fairly busy life.
I'll try to respond quickly, but have some patience.

Also, please don't expect a bounty.
I'd like to provide one, but I can't really afford to.
The best I can guarantee is credit for finding the bug.

## Security

### Philosophy / Threat Model

Heart was primarily designed to counter remote attackers, malicious apps, and data leakage from a permanently lost phone.
You control which apps have access to which information and at what times, down to limiting what domains or IP addresses an app can talk to.
As mentioned above, Heart explicitly compromises on usability for greater security.
Part of that is requiring you to get familiar with the apps you want to run and exactly what they should be allowed to do.
It also means that app developers will need to spend more time than they would on, say, Android, thinking about exactly what their app really needs access to.

Attackers who have physical access _who can get you to take the phone back_ are much harder to defeat in Heart, because of PinePhone.
For more information, see **§Known Issues**.

Heart makes no attempt to enforce secure coding within apps.
However, you have a wide array of tools available to you:
Modern languages, like Rust, which suffer less from common vulnerabilities;
automated vuln detection through fuzzing and static analysis;
turning on compiler warnings.
Use them!
At the very least, if your app is exploitable, Heart won't allow the rest of the system to be exploited too.

### Details

The security of each half of the project is documented respectively in the [happ README] and [kernel README].
Otherwise, this section would get incredibly large.

Unfortunately, because of how interconnected this system is, there isn't really a good starting point.
Each section sort of assumes you've read all the others.
It should start to make sense once you've read them all, but feel free to open an issue if you feel anything is missing.
(If what's missing could have security implications, see **§Disclosure** for the disclosure policy.)

#### Update mechanism

> # TODO

#### App isolation

> # TODO

#### Kernel architecture

> # TODO

#### Syscall interface design

> # TODO

#### App architecture

> # TODO

### Known issues

There are a handful of issues which are known and left open.
The reasoning is explained for each individually.
If you can think of a fix that doesn't encounter the issues mentioned, please raise a feature request or email me at [nic@cybers.eco][0]!

#### SD card boot

PinePhones will, by default, boot from the SD card if one is inserted, before the internal memory.
This will let anyone who can insert an SD card without your knowledge trick you into decrypting your data for them.

This is an easy attack to detect: just pull out any SD cards.
Once the phone as booted, you can reinsert them safely to access the data on them.

At the same time, the only fix would be to modify the bootloader.
A bug there could be catastrophic, permanently bricking the device.
It would also mean that a bricked kernel could, depending on circumstances, become unrecoverable.
Even then, it wouldn't necessarily prevent a sufficiently motivated attacker.

Because the attack is so trivial to defuse manually but so dangerous to do in Heart, I've chosen to leave it unpatched.

## Bugs

> **Note**: If the bug relates to security, even only tangentially, please *send an email instead*.
> See **§Disclosure** for more information.

Is something going wrong?
Are you pretty sure it's not your fault?
Feel free to [report a bug][3]!
Be sure to follow the instructions given there.

Please be aware that this is a side project, and I do have a fairly busy life.
I'll try to respond quickly, but have some patience.

## Features

Is there something... missing?
Something you'd like to see?
[Request a feature][4]!
Be sure to follow the instructions given there.

Please be aware that this is a side project, and I do have a fairly busy life.
I'll try to respond quickly, but have some patience.

 [0]: mailto:nic@cybers.eco
 [1]: https://protonmail.com/security-details
 [2]: https://keybase.io/nichartley
 [3]: https://github.com/nic-hartley/HeartPinePhone/issues/new
 [4]: https://github.com/nic-hartley/HeartPinePhone/issues/new
