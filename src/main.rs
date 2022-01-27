/*!
# Guff
*/

#![forbid(unsafe_code)]

#![warn(clippy::filetype_is_file)]
#![warn(clippy::integer_division)]
#![warn(clippy::needless_borrow)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![warn(clippy::perf)]
#![warn(clippy::suboptimal_flops)]
#![warn(clippy::unneeded_field_pattern)]
#![warn(macro_use_extern_crate)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(non_ascii_idents)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]



mod error;
mod styles;



use argyle::{
	Argue,
	ArgyleError,
	FLAG_HELP,
	FLAG_REQUIRED,
	FLAG_VERSION,
};
use error::GuffError;
use fyi_msg::Msg;
use std::{
	ffi::OsStr,
	os::unix::ffi::OsStrExt,
};



/// Main.
fn main() {
	match _main() {
		Ok(_) => {},
		Err(GuffError::Argue(ArgyleError::WantsVersion)) => {
			println!(concat!("Guff v", env!("CARGO_PKG_VERSION")));
		},
		Err(GuffError::Argue(ArgyleError::WantsHelp)) => { helper(); },
		Err(e) => { Msg::error(e.to_string()).die(1); },
	}
}

#[inline]
/// Actual Main.
fn _main() -> Result<(), GuffError> {
	// Parse CLI arguments.
	let args = Argue::new(FLAG_HELP | FLAG_REQUIRED | FLAG_VERSION)?;

	// Do we just want to generate a config?
	let input = args.option2(b"-i", b"--input").ok_or(GuffError::NoSource)?;
	let code = styles::parse(input)?;

	// Save it!
	if let Some(path) = args.option2(b"-o", b"--output").map(OsStr::from_bytes) {
		write_atomic::write_file(path, code.as_bytes())
			.map_err(|_| GuffError::Write)?;
	}
	// Print it!
	else {
		println!("{}", code);
	}

	Ok(())
}

#[allow(clippy::non_ascii_literal)] // Doesn't work with an r"" literal.
#[cold]
/// Print Help.
fn helper() {
	println!(concat!(
		r"
     __,---.__
  ,-'         `-.__
&/           `._\ _\
/               ''._    ", "\x1b[38;5;199mGuff\x1b[0;38;5;69m v", env!("CARGO_PKG_VERSION"), "\x1b[0m", r"
|   ,             (∞)   SCSS parsing, CSS minifying
|__,'`-..--|__|--''     rolled into one simple app.

USAGE:
    guff [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Print help information and exit.
    -V, --version     Print version information and exit.

OPTIONS:
    -i, --input <FILE>    The path to an SCSS or CSS source file.
    -o, --output <FILE>   The path to save the minified output to. If omitted,
                          the result will be printed to STDOUT instead.
"
	));
}
