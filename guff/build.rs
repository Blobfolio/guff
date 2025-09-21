/*!
# Guff: Build
*/

use dactyl::{
	NiceSeparator,
	NiceU32,
};
use oxford_join::JoinFmt;
use serde::Deserialize;
use std::{
	collections::BTreeMap,
	fmt,
	fs::File,
	io::Write,
	num::NonZeroU32,
	path::PathBuf,
};



/// # Browser Data.
const DATA: &str = "skel/data-2.0.json";



/// # Set Up CLI Arguments.
fn main() {
	println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
	println!("cargo:rerun-if-changed=skel/data-2.0.json");

	build_browser_data();
}

/// # Build Browser/Target Data.
fn build_browser_data() {
	let raw: Raw = serde_json::from_slice(&fetch()).expect("Unable to parse raw.");
	let out: String = process(raw);

	let cache = out_path("guff-browsers.rs");
	File::create(cache)
		.and_then(|mut f| f.write_all(out.as_bytes()).and_then(|_| f.flush()))
		.expect("Unable to save browser data.");
}

/// # Download/Cache Raw JSON.
fn fetch() -> Vec<u8> {
	std::fs::read(DATA).expect("Unable to load browser data.")
}

/// # Output Path.
///
/// Append the sub-path to OUT_DIR and return it.
fn out_path(stub: &str) -> PathBuf {
	std::fs::canonicalize(std::env::var("OUT_DIR").expect("Missing OUT_DIR."))
		.expect("Missing OUT_DIR.")
		.join(stub)
}

/// # Parse Version.
///
/// This parses a version from a string, generating the packed `u32` Parcel
/// expects, and also returning the major for reference.
fn parse_version(src: &str) -> Option<(u32, u32)> {
	use dactyl::traits::BytesToUnsigned;

	// Strip the revision and split on dots.
	let mut version = src.split('-')
		.next()?
		.split('.');

	// Major is first and required.
	let major = version.next().and_then(|v| u32::btou(v.as_bytes()))?;

	// Minor and patch follow, but may be zero.
	let minor = version.next().and_then(|v| u32::btou(v.as_bytes())).unwrap_or(0);
	let patch = version.next().and_then(|v| u32::btou(v.as_bytes())).unwrap_or(0);

	let v: u32 = ((major & 0xff) << 16) | ((minor & 0xff) << 8) | (patch & 0xff);
	let v = NonZeroU32::new(v)?;

	Some((v.get(), major))
}

/// # Process the Data.
fn process(raw: Raw) -> String {
	use fmt::Write;

	let all: BTreeMap<Agent, Vec<Versions>> = raw.agents.into_iter()
		.filter_map(|(k, mut v)| {
			let agent = Agent::try_from(k.as_str()).ok()?;
			v.version_list.sort_by(|a, b| b.era.cmp(&a.era));

			let releases: Vec<Versions> = v.version_list.into_iter()
				.filter_map(|v2| {
					v2.release_date?;
					let (parcel, major) = parse_version(&v2.version)?;
					Some(Versions(parcel, major))
				})
				.collect();

			Some((agent, releases))
		})
		.collect();

	let mut out = String::with_capacity(256 * all.len()); // Just a guess.
	for (k, v) in all {
		writeln!(
			&mut out,
			"#[expect(clippy::missing_docs_in_private_items, reason = \"List is auto-generated.\")]\nconst {}: [(u32, u32); {}] = [{}];",
			k.as_str(),
			v.len(),
			JoinFmt::new(v.into_iter(), ", "),
		).unwrap();
	}
	out
}



#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
/// # Agent Kind.
enum Agent {
	Android,
	Chrome,
	Edge,
	Firefox,
	Ie,
	Ios,
	Opera,
	Safari,
	Samsung,
}

impl Agent {
	const fn as_str(self) -> &'static str  {
		match self {
			Self::Android => "ANDROID",
			Self::Chrome => "CHROME",
			Self::Edge => "EDGE",
			Self::Firefox => "FIREFOX",
			Self::Ie => "IE",
			Self::Ios => "IOS",
			Self::Opera => "OPERA",
			Self::Safari => "SAFARI",
			Self::Samsung => "SAMSUNG",
		}
	}
}

impl TryFrom<&str> for Agent {
	type Error = ();

	fn try_from(src: &str) -> Result<Self, Self::Error> {
		match src.trim() {
			"android" => Ok(Self::Android),
			"chrome" => Ok(Self::Chrome),
			"edge" => Ok(Self::Edge),
			"firefox" => Ok(Self::Firefox),
			"ie" => Ok(Self::Ie),
			"ios_saf" => Ok(Self::Ios),
			"opera" => Ok(Self::Opera),
			"safari" => Ok(Self::Safari),
			"samsung" => Ok(Self::Samsung),
			_ => Err(()),
		}
	}
}



#[derive(Deserialize)]
/// # Agents.
struct Raw {
	agents: BTreeMap<String, RawAgent>
}

#[derive(Deserialize)]
/// # Agent.
struct RawAgent {
	version_list: Vec<RawAgentVersions>,
}

#[derive(Default, Deserialize)]
#[serde(default)]
/// # Agent Version List.
struct RawAgentVersions {
	version: String,
	release_date: Option<u32>,
	era: i32,
}



#[derive(Clone, Copy)]
/// # Versions.
///
/// This is just a tuple, but gives us control over the (codegen) `Display`.
struct Versions(u32, u32);

impl fmt::Display for Versions {
	#[inline]
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(
			f,
			"({}, {})",
			NiceU32::with_separator(self.0, NiceSeparator::Underscore),
			NiceU32::with_separator(self.1, NiceSeparator::Underscore),
		)
	}
}
