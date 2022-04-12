/*!
# Guff: Styles!
*/

use crate::GuffError;
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



/// # Load/Make CSS.
///
/// This method will load the input and either return it as-is — if it is CSS —
/// or build it — if it is SCSS.
///
/// ## Errors
///
/// This will return an error if the file is invalid, unreadable, or does not
/// end with `.css`, `.sass`, or `.scss`.
pub(super) fn css<P>(src: P) -> Result<String, GuffError>
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
			.chars()
			.filter(|x| '\u{feff}'.ne(x))
			.collect(),
	};

	css.trim_mut();
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
pub(super) fn minify<P>(src: P, css: &str, browsers: Option<Browsers>)
-> Result<String, GuffError>
where P: AsRef<Path> {
	// Easy abort.
	if css.is_empty() {
		return Ok(String::new());
	}

	// The path shouldn't be needed, but is requested, so just in case.
	let src: String = std::fs::canonicalize(src)
		.map_err(|_| GuffError::NoSource)?
		.to_string_lossy()
		.into_owned();

	// Parse the stylesheet as CSS.
	let mut stylesheet = StyleSheet::parse(
		src,
		css,
		ParserOptions {
			nesting: true,
			css_modules: false,
			custom_media: false,
			source_index: 0,
		},
	)?;

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
			match src[len - 4] {
				// Maybe CSS?
				b'.' => if src[len - 3].eq_ignore_ascii_case(&b'c') {
					return Ok(Self::Css);
				},
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
