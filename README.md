# Guff

[![Build Status](https://github.com/Blobfolio/guff/workflows/Build/badge.svg)](https://github.com/Blobfolio/guff/actions)
[![Dependency Status](https://deps.rs/repo/github/blobfolio/guff/status.svg)](https://deps.rs/repo/github/blobfolio/guff)

Guff is an SCSS parser and CSS minifier rolled into one.



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
    --release
```

(This should work under other 64-bit Unix environments too, like MacOS.)



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