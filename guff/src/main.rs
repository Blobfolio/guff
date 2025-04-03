/*!
# Guff
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



use argyle::Argument;
use fyi_msg::{
	AnsiColor,
	Msg,
};
use guff_css::{
	Agents,
	Css,
	GuffError,
};
use std::{
	path::Path,
	process::ExitCode,
};



/// # Main.
fn main() -> ExitCode {
	match main__() {
		Ok(()) => ExitCode::SUCCESS,
		Err(GuffError::PrintHelp) => {
			helper();
			ExitCode::SUCCESS
		},
		Err(GuffError::PrintVersion) => {
			println!(concat!("Guff v", env!("CARGO_PKG_VERSION")));
			ExitCode::SUCCESS
		},
		Err(e) => {
			Msg::error(e.to_string()).eprint();
			ExitCode::FAILURE
		},
	}
}

#[inline]
/// # Actual Main.
fn main__() -> Result<(), GuffError> {
	// Parse CLI arguments.
	let args = argyle::args()
		.with_keywords(include!(concat!(env!("OUT_DIR"), "/argyle.rs")));

	let mut expanded = false;
	let mut browsers = None;
	let mut input = None;
	let mut output = None;
	for arg in args {
		match arg {
			Argument::Key("-e" | "--expanded") => { expanded = true; },
			Argument::Key("-h" | "--help") => return Err(GuffError::PrintHelp),
			Argument::Key("-V" | "--version") => return Err(GuffError::PrintVersion),

			Argument::KeyWithValue("-b" | "--browsers", s) => { browsers.replace(s); },
			Argument::KeyWithValue("-i" | "--input", s) => { input.replace(s); },
			Argument::KeyWithValue("-o" | "--output", s) => { output.replace(s); },

			// Nothing else is expected.
			Argument::Other(s) => return Err(GuffError::Cli(s)),
			Argument::InvalidUtf8(s) => return Err(GuffError::Cli(s.to_string_lossy().into_owned())),
			_ => {},
		}
	}

	// In and out.
	let input = input.ok_or(GuffError::NoSource)?;

	// Minify?
	let css = Css::try_from(Path::new(&input))?;
	let code =
		if expanded { css.take() }
		else {
			let browsers =
				if let Some(b) = browsers {
					let agents = Agents::try_from(b.as_str())?;
					if agents.is_empty() { None }
					else {
						// It's helpful to confirm compatibility is being
						// capped and to what, but not if we're sending the CSS
						// to STDOUT.
						if output.is_some() {
							Msg::new(("Compatibility", AnsiColor::LightMagenta), format!(
								"CSS features capped to {agents}."
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
	else { println!("{code}"); }

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
(`(  /_s))   A simple SASS/SCSS compiler
 (_\/_s))    and CSS parser/minifier.
  (\/))

USAGE:
    guff [FLAGS] [OPTIONS]

FLAGS:
    -e, --expanded    Do not minify CSS.
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
