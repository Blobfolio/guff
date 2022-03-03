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
mod targets;



use argyle::{
	Argue,
	ArgyleError,
	FLAG_HELP,
	FLAG_REQUIRED,
	FLAG_VERSION,
};
use error::GuffError;
use fyi_msg::Msg;
use parcel_css::targets::Browsers;
use std::{
	ffi::OsStr,
	os::unix::ffi::OsStrExt,
};
use targets::{
	Agent,
	BrowserOption,
	BrowserOptions,
};



/// # Main.
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
/// # Actual Main.
fn _main() -> Result<(), GuffError> {
	// Parse CLI arguments.
	let args = Argue::new(FLAG_HELP | FLAG_REQUIRED | FLAG_VERSION)?;

	// Do we just want to generate a config?
	let input = args.option2(b"-i", b"--input").ok_or(GuffError::NoSource)?;
	let browsers = parse_browsers(&args);
	let code = styles::parse(input, browsers)?;

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

/// # Parse Browsers.
fn parse_browsers(args: &Argue) -> Option<Browsers> {
	let mut opts = BrowserOptions::from(
		BrowserOption::range(
			args.option(b"--last").map_or(0, parse_u8),
			args.option(b"--age").map_or(0, parse_u8),
		)
	);

	macro_rules! set {
		($($val:literal $key:ident),+ $(,)?) => ($(
			if let Some(x) = args.option($val).map(parse_u32) {
				opts.set(Agent::$key, BrowserOption::major(x));
			}
		)+);
	}

	set!(
		b"android" Android,
		b"chrome" Chrome,
		b"edge" Edge,
		b"firefox" Firefox,
		b"ie" Ie,
		b"ios" Ios,
		b"opera" Opera,
		b"safari" Safari,
		b"samsung" Samsung,
	);

	opts.build()
}

/// # Parse U8 Bytes.
fn parse_u8(raw: &[u8]) -> u8 {
	std::str::from_utf8(raw)
		.ok()
		.and_then(|n| n.parse::<u8>().ok())
		.unwrap_or_default()
}

/// # Parse U32 Bytes.
fn parse_u32(raw: &[u8]) -> u32 {
	std::str::from_utf8(raw)
		.ok()
		.and_then(|n| n.parse::<u32>().ok())
		.unwrap_or_default()
}


#[allow(clippy::non_ascii_literal)] // Doesn't work with an r"" literal.
#[cold]
/// # Print Help.
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

COMPATIBILITY:
        --last <NUM>      Set compatibility to the last X versions of each
                          browser, except IE.
        --age <YRS>       Set compatibility to versions released within the
                          past X years, except IE.
        --android <VER>   Minimum supported version of the Android browser.
        --chrome <VER>    Minimum supported version of Google Chrome.
        --edge <VER>      Minimum supported version of Microsoft Edge.
        --firefox <VER>   Minimum supported version of Firefox.
        --ie <VER>        Minimum supported version of IE. If you want any IE
                          support at all, this *must* be set.
        --ios <VER>       Minimum supported version of iOS (Safari).
        --opera <VER>     Minimum supported version of Opera.
        --safari <VER>    Minimum supported version of Safari.
        --samsung <VER>   Minimum supported version of Samsung's Android
                          browser.

The --last and --age options set the default for all browsers (except IE). If
both are set, only versions meeting *both* criteria will be included.

The per-browser minimum version options expect the major portion of the version
string (e.g. the '1' in '1.2.3'). Explicit minimums will override any last/age
filters.
"
	));
}
