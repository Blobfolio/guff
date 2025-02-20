# Guff

[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/guff/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/guff/actions)
[![deps.rs](https://deps.rs/repo/github/blobfolio/guff/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/repo/github/blobfolio/guff)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/guff/issues)

Guff is an x86-64 Linux CLI tool providing both SASS/SCSS compilation and CSS parsing/minification. It is a faster, lighter, compressier alternative to chaining together multiple tools like `sassc` and `csso`.

Under the hood, it merely marries the SCSS functionality of [grass](https://github.com/connorskees/grass) with the compressive capabilities of [Lightning CSS](lightningcss). If Guff is too opinionated for you or incompatible with your platform, take a look at those projects; they both have frontends of their own. ;)



## Usage

It's easy; just give it a source and, optionally, a destination:
```bash
guff [FLAGS] [OPTIONS]
```

**Flags:**
```text
-h, --help        Print help information and exit.
-V, --version     Print version information and exit.
```

**Options:**
```text
-b, --browsers <STR>  A comma-separated list of specific browser/version pairs
                      to target for CSS compatibility, like 'firefox 90, ie
                      11'. Specifying versions released after guff was built
                      has no effect.
-i, --input <FILE>    The path to an SCSS or CSS source file.
-o, --output <FILE>   The path to save the minified output to. If omitted,
                      the result will be printed to STDOUT instead.
```

If the input is SCSS, it will be compiled into CSS and then minified; if it is already CSS, it will just be minified.



## Installation

Debian and Ubuntu users can just grab the pre-built `.deb` package from the [latest release](https://github.com/Blobfolio/guff/releases/latest).

This application is written in [Rust](https://www.rust-lang.org/) and can alternatively be built/installed from source using [Cargo](https://github.com/rust-lang/cargo):

```bash
# See "cargo install --help" for more options.
cargo install \
    --git https://github.com/Blobfolio/guff.git \
    --bin guff
```

(This should work under other 64-bit Unix environments too, like MacOS.)
