/*!
# Guff Core

This is the backing library for the Guff command-line SCSS/CSS parser and
minifier. It can be used to compile SCSS into CSS, and/or heavily minify CSS
for production use.
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

#[cfg(feature = "bin")]
use fyi_msg as _;

#[cfg(feature = "bin")]
use write_atomic as _;
