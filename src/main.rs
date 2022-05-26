/*!
# Guff
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
	unused_extern_crates,
	unused_import_braces,
)]



use argyle::{
	Argue,
	ArgyleError,
	FLAG_HELP,
	FLAG_REQUIRED,
	FLAG_VERSION,
};
use fyi_msg::Msg;
use guff::{
	Agents,
	Css,
	GuffError,
};
use std::{
	ffi::OsStr,
	path::Path,
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

	// In and out.
	let input = args.option2_os(b"-i", b"--input").ok_or(GuffError::NoSource)?;
	let output = args.option2_os(b"-o", b"--output");

	// Minify?
	let css = Css::try_from(Path::new(input))?;
	let code =
		if args.switch2(b"-e", b"--expanded") { css.take() }
		else {
			let browsers =
				if let Some(b) = args.option2_os(b"-b", b"--browsers").and_then(OsStr::to_str) {
					let agents = Agents::try_from(b)?;
					if agents.is_empty() { None }
					else {
						// It's helpful to confirm compatibility is being
						// capped and to what, but not if we're sending the CSS
						// to STDOUT.
						if output.is_some() {
							Msg::custom("Compatibility", 13, &format!(
								"CSS features capped to {}.", agents
							))
								.with_newline(true)
								.print();
						}

						Some(agents)
					}
				}
				else { None };

			css.minified(browsers)
		}?;

	// Save it!
	if let Some(path) = output {
		write_atomic::write_file(path, code.as_bytes())
			.map_err(|_| GuffError::Write)?;
	}
	// Print it!
	else { println!("{}", code); }

	Ok(())
}

#[cold]
/// # Print Help.
fn helper() {
	println!(concat!(
		r"
   \``/
   /o `))
  /_/\_ss))
      |_ss))/|
     |__ss))_|
    |__sss))_|
    |___ss))\|
     |_ss))
      )_s))  ", "\x1b[38;5;199mGuff\x1b[0;38;5;69m v", env!("CARGO_PKG_VERSION"), "\x1b[0m", r"
(`(  /_s))   SCSS parsing, CSS minifying.
 (_\/_s))
  (\/))

USAGE:
    guff [FLAGS] [OPTIONS]

FLAGS:
    -h, --help        Print help information and exit.
    -V, --version     Print version information and exit.

OPTIONS:
    -b, --browsers <STR>  A comma-separated list of specific browser/version
                          pairs to target for CSS compatibility, like
                          'firefox 90, ie 11'. Specifying versions released
                          after guff was built has no effect.
    -i, --input <FILE>    The path to an SCSS or CSS source file.
    -o, --output <FILE>   The path to save the minified output to. If omitted,
                          the result will be printed to STDOUT instead.

COMPATIBILITY:
    The following browser strings are supported by the -b/--browsers option:
      * android ", "\x1b[2m(the generic Android browser)\x1b[0m
      * chrome
      * edge
      * firefox
      * ios \x1b[2m(mobile Safari)\x1b[0m
      * opera
      * safari
      * samsung \x1b[2m(Samsung's Android browser)\x1b[0m
"
	));
}
