# `minigrep`

![Build](https://github.com/heuristic-pedals/minigrep/actions/workflows/ci.yml/badge.svg?branch=main)
![Docs](https://github.com/heuristic-pedals/minigrep/actions/workflows/docs.yml/badge.svg?branch=main)
![Codecov](https://img.shields.io/codecov/c/github/heuristic-pedals/minigrep)

A small [grep][grep]-like implementation in `rust`. Searches for a sub-string pattern (not regex) within lines of a single file and returns them.

## Introduction

This repo contains both a binary and library crate. The binary crate can be built and executed locally via the command line interface - this process is described below. The public API of the library is outlined in the relevant section of this site.

## Setup

Prerequisites:
- `git`
- Minimum: `rustc` (recommended: `rustup` + `cargo`)

To build the binary, an installation of `rustc` is required. The following example uses `cargo`, but this isn't strictly required. See the [rustc dev guide][rustc-dev-guide] for more details on building using `rustc` directly.

Execute the following commands to set-up and build the `minigrep` binary (optimised in release mode):

```shell
git clone https://github.com/heuristic-pedals/minigrep.git
cd minigrep
cargo build --release
```

## Usage

The command line interface can be used as follows:

```shell
./target/release/minigrep [SUB-STRING] [PATH-TO-FILE]
```

Where:

- `[SUB-STRING]`: the sub-string pattern to search for.
- `[PATH-TO-FILE]`: path to the file to search within.

### Usage Example

Say we are interested in finding the sub-string `'duct'` within the following `.txt` file located within a sub-directory `./data/`

> Rust:<br>
  Safe, fast, productive.<br>
  Pick three.<br>
  Duct tape.<br>

This can be done as follows:

```shell
> minigrep % ./target/release/minigrep duct data/example.txt
L2: safe, fast, productive.
```

The output returns the line number (in this example just L2) and all of the corresponding line's content. Note, the default behaviour assumes case sensitivity (hence line 4 is not returned). Case insensitive scenarios can be considered by setting the `IGNORE_CASE` environment variable (then line 4 would also be returned, see the [Environment Variable][#environment-variables] section for more details).

### Environment Variables

`IGNORE_CASE`:

When set, this will ignore case - treating upper and lower case characters as equivalents. This can be set to any value, it just needs to exist within the scope of the active session. For example, continuing the example from above, on a UNIX machine this can be done easily for a single execution as follows:

```shell
> minigrep % IGNORE_CASE=1 ./target/release/minigrep duct data/example.txt
L2: safe, fast, productive.
L4: Duct tape.
```

## Licence

This code is released under the [MIT Licence][mit].

## Notes

These binary and libary crates are based on [Chapter 12 of The Rust Book][rust book]. This repository exists for personal practice and learning purposes only, whilst following this book. However, changes and additional functionalities beyond The Rust Book have been introduced such as; refactoring improvements, further unit tests, integration tests, code coverage using [`cargo-llvm-cov`][cargo-llvm-cov] + [codecov][codecov], and documentation (using `cargo doc`). To prevent cluttering [`crates.io`][crates-io], the source code and documentation is only hosted in this repository.


[grep]: https://www.gnu.org/software/grep/manual/grep.html
[rust book]: https://doc.rust-lang.org/book/ch12-00-an-io-project.html
[rustc-dev-guide]: https://rustc-dev-guide.rust-lang.org/building/how-to-build-and-run.html
[cargo-llvm-cov]: https://github.com/taiki-e/cargo-llvm-cov
[codecov]: https://about.codecov.io
[crates-io]: https://crates.io
[mit]: LICENSE