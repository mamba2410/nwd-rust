# nwd-rust

This is a rewrite of mamba2410/nwd-2 in the Rust programming language.

The purpose of this program is to be able to easily create a project dev environment easily for different languages.

It is based around the C programming language however custom languages can be added easily with no modification to the main binary.
Shell scripts are run for each language and are designes to be like a 'patch' to the base C environment.
So the more the language environment differs from C, the more complex the shell script will be.

It is intended to be platform-independent and the main 'version' of `nwd` from now on.

## Out-of-the-box
Support for the following languages:
- C (default) (`c`)
- Assembly, based on x86\_64 but can be modified for aarch64 or others (`asm`)
- C with assembly (`c+asm`)
- C++ (`cpp`)
- Fortran-90 (`f90`)
- Arduino, almost the same as Arduino IDE (`arduino`)
- C on ATMEL chips, aimed for arduino (`arduino-raw`)
- Rust, probably better to use `cargo` (`rust`)
- LaTeX, might be iffy and it is highly personal (`latex`)

The following licenses are available by default: 
- GPL-3 (`GPL3`)
- GPL-2 (`GPL2`)
- MIT (`MIT`)
- 3-Clause BSD (`BSD3`)


## Installation
The program requires its support files to be in `$XDG_DATA_HOME/nwd` however if this directory does not exist, it defaults to `~/nwd`.

To install:
```
$ git clone https://github.com/mamba2410/nwd-rust
$ cargo build --release
$ mv nwd-rust $XDG_DATA_HOME/nwd
```
Then add the `target/release/nwd` binary to a directory in your `$PATH` variable.
For example:
```
$ ln -s $XDG_DATA_HOME/nwd/target/release/nwd-rust ~/.local/bin/nwd
```

## Issues
Maybe run into issues on Windows, not been tested much but appears to work okay when running from a shell such as git bash.


## Improvments
I'm still new to Rust so the program structure needs a lot of improving.

- Handle internal rust errors better.
- Collect arguments better, allow things like `-Dgr /path/ro/remote`
- Re-organise program flow and make it more modular

