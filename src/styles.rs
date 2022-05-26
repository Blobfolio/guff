/*!
# Guff: Styles!
*/

use crate::{
	Agents,
	GuffError,
};
use grass::{
	Options,
	OutputStyle,
};
use parcel_css::{
	stylesheet::{
		MinifyOptions,
		ParserOptions,
		PrinterOptions,
		StyleSheet,
	},
	targets::Browsers,
};
use std::{
	os::unix::ffi::OsStrExt,
	path::{
		Path,
		PathBuf,
	},
};
use trimothy::TrimMut;



macro_rules! parse {
	($css:expr) => (
		StyleSheet::parse("stylesheet.css", $css, ParserOptions {
			nesting: true,
			css_modules: None,
			custom_media: false,
			source_index: 0,
		})
	);
}



/// # Load/Make CSS.
///
/// This method loads (and/or compiles) style rules from the given CSS, SASS,
/// or SCSS file, verifies the syntax is valid, and returns the result (with
/// some light cleanup applied).
///
/// Paths ending in `.sass` or `.scss` are assumed to be SCSS, and are run
/// through the compiler to generate valid CSS before doing anything else.
///
/// The CSS is trimmed, stripped of UTF-8 BOM markers, and validated before
/// being returned.
///
/// Note: CSS modules are not supported, but SCSS imports are.
///
/// ## Errors
///
/// This will return an error if the file is invalid, unreadable, or does not
/// end with `.css`, `.sass`, or `.scss`, or if the code cannot be parsed or
/// validated.
pub fn css<P>(src: P) -> Result<String, GuffError>
where P: AsRef<Path> {
	// Make the path sane.
	let src = src.as_ref();
	let path: PathBuf = std::fs::canonicalize(src)
		.ok()
		.filter(|x| x.is_file())
		.ok_or(GuffError::NoSource)?;

	// Both grass and parcel_css require string paths for some reason, so we
	// have to make sure it can be stringified.
	let path_str: &str = path.to_str().ok_or(GuffError::SourceFileName)?;

	// Come up with CSS.
	let mut css: String = match StyleKind::try_from(src.as_os_str().as_bytes())? {
		// The CSS has to be built from SASS.
		StyleKind::Scss => grass::from_path(
			path_str,
			&Options::default()
				.style(OutputStyle::Expanded)
				.quiet(true)
		)?,
		// The file is already CSS; we just need to read it!
		StyleKind::Css => std::fs::read_to_string(&path)
			.map_err(|_| GuffError::SourceInvalid)?
	};

	// Strip out UTF-8 BOM characters.
	css.retain(|c| c != '\u{feff}');

	// Trim it.
	css.trim_mut();

	// Make sure it is parseable.
	if ! css.is_empty() {
		parse!(&css)?;
	}

	Ok(css)
}



/// # Minify CSS.
///
/// This method accepts raw CSS and minifies it, returning the improved
/// version.
///
/// If browser compatibility targets are specified, some advanced compression
/// techniques may be disabled.
///
/// ## Errors
///
/// This will return an error if the document cannot be processed.
pub fn minify(css: &str, browsers: Option<Agents>) -> Result<String, GuffError> {
	// Easy abort.
	if css.is_empty() { Ok(String::new()) }
	else {
		// Parse the stylesheet as CSS.
		let mut stylesheet = parse!(css)?;

		// Convert our Agents into a parcel Browsers object.
		let browsers = browsers.and_then(Option::<Browsers>::from);

		// Minify it.
		stylesheet.minify(MinifyOptions {
			targets: browsers,
			..MinifyOptions::default()
		})?;

		// Turn it back into a string.
		let out = stylesheet.to_css(PrinterOptions {
			minify: true,
			source_map: None,
			targets: browsers,
			..PrinterOptions::default()
		})?;

		// Done!
		Ok(out.code)
	}
}



#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
/// # Style Kind.
enum StyleKind {
	Css,
	Scss,
}

impl TryFrom<&[u8]> for StyleKind {
	type Error = GuffError;

	/// # From Path Name (as bytes).
	///
	/// This just teases out the type based on the extension. We already have
	/// raw bytes, so might as well take advantage of the easy comparisons.
	fn try_from(src: &[u8]) -> Result<Self, Self::Error> {
		let len: usize = src.len();

		// The last two letters should always be ss.
		if 5 < len && matches!(src[len - 2], b's' | b'S') && matches!(src[len - 1], b's' | b'S') {
			match src[len - 4] {
				// Maybe CSS?
				b'.' => if matches!(src[len - 3], b'c' | b'C') { return Ok(Self::Css); },
				// Maybe SCSS/SASS?
				b's' | b'S' => if src[len - 5] == b'.' && matches!(src[len - 3], b'a' | b'c' | b'A' | b'C') {
					return Ok(Self::Scss);
				},
				_ => {},
			}
		}

		Err(GuffError::SourceInvalid)
	}
}



#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn t_stylekind() {
		for (file, expected) in [
			("/foo/bar.css", Some(StyleKind::Css)),
			("/foo/bar.sass", Some(StyleKind::Scss)),
			("/foo/bar.scss", Some(StyleKind::Scss)),
			("/foo/bar.jpeg", None),
		] {
			assert_eq!(StyleKind::try_from(file.as_bytes()).ok(), expected);
			let file = file.to_uppercase();
			assert_eq!(StyleKind::try_from(file.as_bytes()).ok(), expected);
		}
	}
}
