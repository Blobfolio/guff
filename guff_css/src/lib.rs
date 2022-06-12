/*!
# Guff CSS

[![Documentation](https://docs.rs/guff_css/badge.svg)](https://docs.rs/guff_css/)
[![crates.io](https://img.shields.io/crates/v/guff_css.svg)](https://crates.io/crates/guff_css)
[![Build Status](https://github.com/Blobfolio/guff/workflows/Build/badge.svg)](https://github.com/Blobfolio/guff/actions)

This is the backing library for the Guff command-line SCSS/CSS parser and
minifier. It can be used to compile SCSS into CSS, and/or heavily minify CSS
for production use.

Refer to the [documentation](https://docs.rs/guff_css/) for usage and other details.

**Compatibility**

This library is only compatible with Unix platforms. For broader support,
consider using [`grass`](https://crates.io/crates/grass) and [`parcel_css`](https://crates.io/crates/parcel_css) directly.

**Work In Progress**

Both the SASS/SCSS compilation and CSS minification features are works in
progress, and might change subtly between releases. To be safe, CSS generated
by Guff should be tested in a staging environment before being pushed to
production.
*/

#![forbid(unsafe_code)]

#![warn(
	clippy::filetype_is_file,
	clippy::integer_division,
	clippy::needless_borrow,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::suboptimal_flops,
	clippy::unneeded_field_pattern,
	macro_use_extern_crate,
	missing_copy_implementations,
	missing_debug_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unreachable_pub,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]

#![allow(clippy::module_name_repetitions)]



mod error;
mod styles;
mod targets;

pub use error::GuffError;
pub use styles::Css;
pub use targets::{
	Agent,
	Agents,
};
