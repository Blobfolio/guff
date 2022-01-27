/*!
# Guff: Errors
*/

use argyle::ArgyleError;
use cssparser::{
	ParseError,
	ParseErrorKind,
};
use parcel_css::error::{
	MinifyError,
	ParserError,
	PrinterError,
};
use std::{
	error::Error,
	fmt,
};



#[derive(Debug, Clone)]
/// # Error type.
pub(super) enum GuffError {
	/// # Argyle passthrough.
	Argue(ArgyleError),

	/// # CSS Parse Error.
	Css(String),

	/// # Non-specific CSS Parse Error.
	CssSimple,

	/// # No Source.
	NoSource,

	/// # SCSS Parse Error.
	Scss(String),

	/// # Source File Name.
	SourceFileName,

	/// # Invalid Source.
	SourceInvalid,

	/// # Write Error.
	Write,
}

impl Error for GuffError {}

impl fmt::Display for GuffError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Css(s) => write!(f, "CSS error: {}", s),
			Self::Scss(s) => write!(f, "SCSS error: {}", s),
			_ => f.write_str(self.as_str()),
		}
	}
}

impl From<ArgyleError> for GuffError {
	#[inline]
	fn from(err: ArgyleError) -> Self { Self::Argue(err) }
}

impl From<Box<grass::Error>> for GuffError {
	#[inline]
	fn from(err: Box<grass::Error>) -> Self { Self::Scss(err.to_string()) }
}

macro_rules! parcel_error {
	($($ty:ty),+) => ($(
		impl From<$ty> for GuffError {
			#[inline]
			fn from(err: $ty) -> Self { Self::Css(err.reason()) }
		}
	)+)
}

parcel_error!(MinifyError, PrinterError, ParserError<'_>);

impl<'a> From<ParseError<'a, ParserError<'a>>> for GuffError {
	fn from(err: ParseError<'a, ParserError<'a>>) -> Self {
		match err.kind {
			ParseErrorKind::Basic(_) => Self::CssSimple,
			ParseErrorKind::Custom(t) => t.into(),
		}
	}
}

impl GuffError {
	/// # As Str.
	pub(super) const fn as_str(&self) -> &'static str {
		match self {
			Self::Argue(e) => e.as_str(),
			Self::CssSimple | Self::Css(_) => "Unable to parse CSS.",
			Self::NoSource => "An SCSS/CSS source is required.",
			Self::Scss(_) => "Unable to parse SCSS.",
			Self::SourceFileName => "File paths must be valid UTF-8.",
			Self::SourceInvalid => "Invalid/unreadable SCSS/CSS source.",
			Self::Write => "The output could not be saved to disk.",
		}
	}
}
