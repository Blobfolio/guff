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
	CssParse(String),
	CssParseSimple,
	/// # No Source.
	NoSource,
	/// # SCSS Parse Error.
	ScssParse(String),
	/// # Invalid Source.
	SourceInvalid,
	/// # Write Error.
	Write,
}

impl Error for GuffError {}

impl fmt::Display for GuffError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::CssParse(s) => write!(f, "CSS error: {}", s),
			Self::ScssParse(s) => write!(f, "SCSS error: {}", s),
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
	fn from(err: Box<grass::Error>) -> Self { Self::ScssParse(err.to_string()) }
}

impl From<MinifyError> for GuffError {
	#[inline]
	fn from(err: MinifyError) -> Self { Self::CssParse(err.reason()) }
}

impl<'a> From<ParseError<'a, ParserError<'a>>> for GuffError {
	fn from(err: ParseError<'a, ParserError<'a>>) -> Self {
		match err.kind {
			ParseErrorKind::Basic(_) => Self::CssParseSimple,
			ParseErrorKind::Custom(t) => t.into(),
		}
	}
}

impl<'a> From<ParserError<'a>> for GuffError {
	#[inline]
	fn from(err: ParserError<'a>) -> Self { Self::CssParse(err.reason()) }
}

impl From<PrinterError> for GuffError {
	#[inline]
	fn from(err: PrinterError) -> Self { Self::CssParse(err.reason()) }
}

impl GuffError {
	/// # As Str.
	pub(super) const fn as_str(&self) -> &'static str {
		match self {
			Self::Argue(e) => e.as_str(),
			Self::CssParseSimple | Self::CssParse(_) => "Unable to parse CSS.",
			Self::NoSource => "An SCSS/CSS source is required.",
			Self::ScssParse(_) => "Unable to parse SCSS.",
			Self::SourceInvalid => "Invalid/unreadable SCSS/CSS source.",
			Self::Write => "The output could not be saved to disk.",
		}
	}
}
