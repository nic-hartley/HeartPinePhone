# HeartPinePhone

**HeartPinePhone**, sometimes **Heart** or **HPP** where unambiguous, is a custom kernel for the PinePhone.
It's mostly a personal project, but if you want to see the beginnings of an actual pitch for it, check out the history of this README.
It's meant to be a minimal kernel, built on modular code so it's easy to audit.
It also has a robust permission system, which includes basics like internet access.
Perhaps most importantly, I'm writing it!
That makes it fun, and therefore worthwhile.

This repo is split into four sections:

- **aarch64-happ**, a build target to compile Rust code for HeartPinePhone.
- **cargo-happ**, a custom Cargo subcommand to build HeartPinePhone apps.
  `aarch64-happ` will get you an executable built, but `cargo-happ` is how you sign it, compress it, etc.
- **kernel**, the kernel itself, plus a build script to get it into U_Boot format.

For more info on each, see their respective READMEs.

There's also [the devblog]!

  [the devblog]: https://redfennec.dev/hpp
