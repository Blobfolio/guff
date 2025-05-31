/*!
# Guff CSS

[![docs.rs](https://img.shields.io/docsrs/guff_css.svg?style=flat-square&label=docs.rs)](https://docs.rs/guff_css/)
[![changelog](https://img.shields.io/crates/v/guff_css.svg?style=flat-square&label=changelog&color=9b59b6)](https://github.com/Blobfolio/guff/blob/master/guff_css/CHANGELOG.md)<br>
[![crates.io](https://img.shields.io/crates/v/guff_css.svg?style=flat-square&label=crates.io)](https://crates.io/crates/guff_css)
[![ci](https://img.shields.io/github/actions/workflow/status/Blobfolio/guff/ci.yaml?style=flat-square&label=ci)](https://github.com/Blobfolio/guff/actions)
[![deps.rs](https://deps.rs/crate/guff_css/latest/status.svg?style=flat-square&label=deps.rs)](https://deps.rs/crate/guff_css/)<br>
[![license](https://img.shields.io/badge/license-wtfpl-ff1493?style=flat-square)](https://en.wikipedia.org/wiki/WTFPL)
[![contributions welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat-square&label=contributions)](https://github.com/Blobfolio/guff/issues)

This is the backing library for the Guff command-line SCSS/CSS parser and
minifier. It can be used to compile SCSS into CSS, and/or heavily minify CSS
for production use.

Refer to the [documentation](https://docs.rs/guff_css/) for usage and other details.

**Compatibility**

This library is only compatible with Unix platforms. For broader support,
consider using [`grass`](https://crates.io/crates/grass) and [`lightningcss`](https://crates.io/crates/lightningcss) directly.

**Work In Progress**

Both the SASS/SCSS compilation and CSS minification features are works in
progress, and might change subtly between releases. To be safe, CSS generated
by Guff should be tested in a staging environment before being pushed to
production.
*/

#![forbid(unsafe_code)]

#![deny(
	clippy::allow_attributes_without_reason,
	clippy::correctness,
	unreachable_pub,
)]

#![warn(
	clippy::complexity,
	clippy::nursery,
	clippy::pedantic,
	clippy::perf,
	clippy::style,

	clippy::allow_attributes,
	clippy::clone_on_ref_ptr,
	clippy::create_dir,
	clippy::filetype_is_file,
	clippy::format_push_string,
	clippy::get_unwrap,
	clippy::impl_trait_in_params,
	clippy::lossy_float_literal,
	clippy::missing_assert_message,
	clippy::missing_docs_in_private_items,
	clippy::needless_raw_strings,
	clippy::panic_in_result_fn,
	clippy::pub_without_shorthand,
	clippy::rest_pat_in_fully_bound_structs,
	clippy::semicolon_inside_block,
	clippy::str_to_string,
	clippy::string_to_string,
	clippy::todo,
	clippy::undocumented_unsafe_blocks,
	clippy::unneeded_field_pattern,
	clippy::unseparated_literal_suffix,
	clippy::unwrap_in_result,

	macro_use_extern_crate,
	missing_copy_implementations,
	missing_docs,
	non_ascii_idents,
	trivial_casts,
	trivial_numeric_casts,
	unused_crate_dependencies,
	unused_extern_crates,
	unused_import_braces,
)]



mod error;
mod styles;
mod targets;

pub use error::GuffError;
pub use styles::Css;
pub use targets::{
	Agent,
	Agents,
};
