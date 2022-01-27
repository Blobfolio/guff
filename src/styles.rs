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

	// Build or read the CSS.
	let css: String = match StyleKind::try_from(src)? {
		StyleKind::Scss => {
			let opts = Options::default()
				.style(OutputStyle::Expanded)
				.quiet(true);

			grass::from_path(
				path.to_str().ok_or(GuffError::SourceFileName)?,
				&opts
			)
				.map_err(GuffError::from)?
		},
		StyleKind::Css => {
			let mut css: String = std::fs::read_to_string(&path)
				.map_err(|_| GuffError::SourceInvalid)?;

			// Make sure there is no UTF-8 BOM, as it can cause problems with
			// inlined styles.
			if css.len() > 3 {
				let v = unsafe { css.as_mut_vec() };
				if v[0] == 0xef && v[1] == 0xbb && v[2] == 0xbf { v.drain(..3); }
			}

			css
		},
	};

	// Parse the stylesheet as CSS.
	let mut stylesheet = StyleSheet::parse(
		path.to_str().ok_or(GuffError::SourceInvalid)?.to_string(),
		&css,
		ParserOptions {
			nesting: true,
			css_modules: false,
			custom_media: false,
		},
	)
		.map_err(GuffError::from)?;

	// Minify it.
	stylesheet.minify(MinifyOptions::default())?;

	// Turn it back into a string.
	stylesheet.to_css(PrinterOptions {
		minify: true,
		source_map: false,
		..PrinterOptions::default()
	})
		.map(|x| x.code)
		.map_err(GuffError::from)
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
				if src[len - 4].eq_ignore_ascii_case(&b's') {
					let mid = src[len - 3].to_ascii_lowercase();
					if mid == b'a' || mid == b'c' {
						return Ok(Self::Scss);
					}
				}
			}
			// A three-letter extension could be CSS.
			else if src[len - 4] == b'.' && src[len - 3].eq_ignore_ascii_case(&b'c') {
				return Ok(Self::Css);
			}
		}

		Err(GuffError::SourceInvalid)
	}
}
