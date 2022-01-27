/*!
# Guff: Styles!
*/

use crate::GuffError;
use grass::{
	Options,
	OutputStyle,
};
use parcel_css::stylesheet::{
	MinifyOptions,
	ParserOptions,
	PrinterOptions,
	StyleSheet
};
use std::{
	ffi::OsStr,
	os::unix::ffi::OsStrExt,
	path::PathBuf,
};



/// # Parse and Process!
///
/// This method takes a file path — represented as bytes because that's what
/// we've got — and parses, processes, and minifies the data, returning a
/// minified CSS copy as a string if all went well.
///
/// If the source has a `.sass` or `.scss` extension, it will first be parsed
/// into raw CSS. If the source is already `.css`, that step is skipped.
///
/// ## Errors
///
/// This will return an error if the file is invalid, unreadable, or
/// unparseable.
pub(super) fn parse(src: &[u8]) -> Result<String, GuffError> {
	// Make the path sane.
	let path: PathBuf = std::fs::canonicalize(OsStr::from_bytes(src))
		.ok()
		.filter(|x| x.is_file())
		.ok_or(GuffError::NoSource)?;

	// Both grass and parcel_css require string paths for some reason, so we
	// have to make sure it can be stringified.
	let path_str: &str = path.to_str().ok_or(GuffError::SourceFileName)?;

	// Come up with CSS.
	let css: String = match StyleKind::try_from(src)? {
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
			.chars()
			.filter(|x| '\u{feff}'.ne(x))
			.collect(),
	};

	// Easy abort.
	if css.trim().is_empty() {
		return Ok(String::new());
	}

	// Parse the stylesheet as CSS.
	let mut stylesheet = StyleSheet::parse(
		path_str.to_string(),
		&css,
		ParserOptions {
			nesting: true,
			css_modules: false,
			custom_media: false,
		},
	)?;

	// Minify it.
	stylesheet.minify(MinifyOptions::default())?;

	// Turn it back into a string.
	let out = stylesheet.to_css(PrinterOptions {
		minify: true,
		source_map: false,
		..PrinterOptions::default()
	})?;

	// Done!
	Ok(out.code)
}



#[derive(Debug, Clone, Copy)]
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
		if len > 5 && src[len - 2..].eq_ignore_ascii_case(b"ss") {
			// A four-letter extension could be Sass.
			if src[len - 5] == b'.' {
				if
					(src[len - 4] == b's' || src[len - 4] == b'S') &&
					matches!(src[len - 3], b'a' | b'c' | b'A' | b'C')
				{
					return Ok(Self::Scss);
				}
			}
			// A three-letter extension could be CSS.
			else if src[len - 4] == b'.' && (src[len - 3] == b'c' || src[len - 3] == b'C') {
				return Ok(Self::Css);
			}
		}

		Err(GuffError::SourceInvalid)
	}
}
