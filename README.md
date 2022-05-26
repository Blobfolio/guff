# Guff

[![Documentation](https://docs.rs/guff/badge.svg)](https://docs.rs/guff/)
[![crates.io](https://img.shields.io/crates/v/guff.svg)](https://crates.io/crates/guff)
[![Build Status](https://github.com/Blobfolio/guff/workflows/Build/badge.svg)](https://github.com/Blobfolio/guff/actions)
[![Dependency Status](https://deps.rs/repo/github/blobfolio/guff/status.svg)](https://deps.rs/repo/github/blobfolio/guff)

Guff is a SASS/SCSS parser and CSS minifier rolled into one.

Note: feature-complete SASS/SCSS parsing is still a work in progress. Known issues are tracked [here](https://github.com/connorskees/grass/issues/19).



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

If the input is SCSS, it will be processed into CSS and then minified; if it is already CSS, it will just be minified.



## Installation

Debian and Ubuntu users can just grab the pre-built `.deb` package from the [latest release](https://github.com/Blobfolio/guff/releases/latest).

This application is written in [Rust](https://www.rust-lang.org/) and can alternatively be built from source using [Cargo](https://github.com/rust-lang/cargo):

```bash
# Clone the source.
git clone https://github.com/Blobfolio/guff.git

# Go to it.
cd guff

# Build as usual. Specify additional flags as desired.
cargo build \
    --bin guff \
    --all-features \
    --release
```

(This should work under other 64-bit Unix environments too, like MacOS.)



## Library

Guff can also be used as a Rust library by adding it to your project's `Cargo.toml` like:

```toml
[dependencies]
guff = "0.1"
```

Refer to the [documentation](docs.rs/guff/) for usage and other details.



## License

See also: [CREDITS.md](CREDITS.md)

Copyright © 2022 [Blobfolio, LLC](https://blobfolio.com) &lt;hello@blobfolio.com&gt;

This work is free. You can redistribute it and/or modify it under the terms of the Do What The Fuck You Want To Public License, Version 2.

    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    Version 2, December 2004
    
    Copyright (C) 2004 Sam Hocevar <sam@hocevar.net>
    
    Everyone is permitted to copy and distribute verbatim or modified
    copies of this license document, and changing it is allowed as long
    as the name is changed.
    
    DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
    
    0. You just DO WHAT THE FUCK YOU WANT TO.
