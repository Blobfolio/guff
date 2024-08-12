/*!
# Guff: Build
*/

use dactyl::NiceU32;
use serde::Deserialize;
use std::{
	collections::BTreeMap,
	fs::File,
	io::Write,
	num::NonZeroU32,
	path::PathBuf,
};

#[cfg(not(docsrs))]
use std::{
	fs::Metadata,
	path::Path,
};



#[cfg(not(docsrs))]
/// # Can I Use? publishes their data here.
const DATA_URL: &str = "https://github.com/Fyrd/caniuse/raw/main/fulldata-json/data-2.0.json";

/// # Fallback.
const DATA_FALLBACK: &str = "skel/data-2.0.json";



/// # Pull (supported) browser version data from Can I Use.
///
/// This ultimately compiles a static array of (Kind, Tuple) data, where each
/// tuple holds:
/// 0. The version as Parcel wants it.
/// 1. The major version number.
/// 2. The release date (UTC).
///
/// The versions are sorted descendingly.
///
/// That's it!
pub fn main() {
	println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");

	let raw: Raw = serde_json::from_slice(&fetch()).expect("Unable to parse raw.");
	let out: String = process(raw);

	let cache = out_path("guff-browsers.rs");
	File::create(cache)
		.and_then(|mut f| f.write_all(out.as_bytes()).and_then(|_| f.flush()))
		.expect("Unable to save browser data.");
}

#[cfg(docsrs)]
/// # Fetch Raw JSON.
///
/// This is a workaround for docs.rs that just pulls a stale copy shipped with
/// the library.
fn fetch() -> Vec<u8> {
	std::fs::read(DATA_FALLBACK).expect("Unable to load browser data.")
}

#[cfg(not(docsrs))]
/// # Download/Cache Raw JSON.
fn fetch() -> Vec<u8> {
	// Is it cached?
	let cache = out_path("guff-browsers.json");
	if let Some(x) = try_cache(&cache) {
		return x;
	}

	fetch_remote(&cache).unwrap_or_else(fetch_local)
}

#[cfg(not(docsrs))]
/// # Fetch Remote.
fn fetch_remote(cache: &Path) -> Option<Vec<u8>> {
	// Download it.
	let res = minreq::get(DATA_URL)
		.with_header("user-agent", "Mozilla/5.0")
		.with_timeout(30)
		.send()
		.ok()?;

	// Only accept happy response codes with sized bodies.
	if (200..=399).contains(&res.status_code) {
		let out = res.into_bytes();
		if ! out.is_empty() {
			// Try to save for next time.
			let _res = File::create(cache)
				.and_then(|mut f| f.write_all(&out).and_then(|_| f.flush()));

			return Some(out);
		}
	}

	None
}

#[cfg(not(docsrs))]
/// # Fetch Local.
fn fetch_local() -> Vec<u8> {
	let out = std::fs::read(DATA_FALLBACK).expect("Unable to load browser data.");
	println!("cargo:warning=Unable to download current caniuse data; building with bundled copy instead.");
	out
}

/// # Process the Data.
fn process(raw: Raw) -> String {
	let all: BTreeMap<Agent, Vec<(u32, u32)>> = raw.agents.into_iter()
		.filter_map(|(k, mut v)| {
			let agent = Agent::try_from(k.as_str()).ok()?;
			v.version_list.sort_by(|a, b| b.era.cmp(&a.era));

			let releases: Vec<(u32, u32)> = v.version_list.into_iter()
				.filter_map(|v2| {
					v2.release_date?;
					let (parcel, major) = parse_version(&v2.version)?;
					Some((parcel, major))
				})
				.collect();

			Some((agent, releases))
		})
		.collect();

	all.into_iter()
		.map(|(k, v)| {
			format!(
				"const {}: [(u32, u32); {}] = [{}];",
				k.as_str(),
				v.len(),
				v.into_iter()
					.map(|(a, b)| format!(
						"({}, {})",
						NiceU32::with_separator(a, b'_'),
						NiceU32::with_separator(b, b'_'),
					))
					.collect::<Vec<String>>()
					.join(", ")
			)
		})
		.collect::<Vec<String>>()
		.join("\n")
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



/// # Out path.
///
/// This generates a (file/dir) path relative to `OUT_DIR`.
fn out_path(name: &str) -> PathBuf {
	let dir = std::env::var("OUT_DIR").expect("Missing OUT_DIR.");
	let mut out = std::fs::canonicalize(dir).expect("Missing OUT_DIR.");
	out.push(name);
	out
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

	let v: u32 = (major & 0xff) << 16 | (minor & 0xff) << 8 | (patch & 0xff);
	let v = NonZeroU32::new(v)?;

	Some((v.get(), major))
}

#[cfg(not(docsrs))]
/// # Try Cache.
///
/// The downloaded files are cached locally in the `target` directory, but we
/// don't want to run the risk of those growing stale if they persist between
/// sessions, etc.
///
/// At the moment, cached files are used if they are less than an hour old,
/// otherwise the cache is ignored and they're downloaded fresh.
fn try_cache(path: &Path) -> Option<Vec<u8>> {
	std::fs::metadata(path)
		.ok()
		.filter(Metadata::is_file)
		.and_then(|meta| meta.modified().ok())
		.and_then(|time| time.elapsed().ok().filter(|secs| secs.as_secs() < 3600))
		.and_then(|_| std::fs::read(path).ok())
}
