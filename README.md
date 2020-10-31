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
- **explorer**, firmware for the PinePhone that I'm using to explore the device.

For more info on each, see their respective READMEs.

There's also [the devblog]!
Notes will generally end up in this repo first, because while my blogposts can be eclectic, I do try to keep them coherent.
That generally means thinking through my ideas and refining them a bit.
The notes here are mostly for me, so I can pick back up on them.
The posts are reserved for when I have a slightly better idea of what I'm thinking about.

## `build-fw.sh`

I've got a shell script to handle all the steps of building, for now.
Eventually it'll be a nice, clean Cargo subcommand, but for now it's... not.

To make it work, you'll need:

- Nightly rust, with the `rust-src` component _on nightly_.
  Install it with `rustup component add rust-src --toolchain nightly`.
- The assembler and objcopy for the right platform.
  To check which you need, look at the project's `compile-target/spec.json`'s `project-platform` value.
  You can generally install a package called `[prefix]-binutils` and be all set.

  [the devblog]: https://redfennec.dev/hpp
