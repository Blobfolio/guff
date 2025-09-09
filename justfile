##
# Development Recipes
#
# This justfile is intended to be run from inside a Docker sandbox:
# https://github.com/Blobfolio/righteous-sandbox
#
# docker run \
#	--rm \
#	-v "{{ invocation_directory() }}":/share \
#	-it \
#	--name "righteous_sandbox" \
#	"righteous/sandbox:debian"
#
# Alternatively, you can just run cargo commands the usual way and ignore these
# recipes.
##

pkg_id      := "guff"
pkg_name    := "Guff"
pkg_dir1    := justfile_directory() + "/guff"
pkg_dir2    := justfile_directory() + "/guff_css"

features    := "bin"

cargo_dir   := "/tmp/" + pkg_id + "-cargo"
cargo_bin   := cargo_dir + "/release/" + pkg_id
data_dir    := "/tmp/bench-data"
doc_dir     := justfile_directory() + "/doc"
release_dir := justfile_directory() + "/release"
skel_dir    := justfile_directory() + "/skel"

caniuse_url := "https://github.com/Fyrd/caniuse/raw/main/fulldata-json/data-2.0.json"
caniuse_tmp := "/tmp/caniuse.json"

export RUSTFLAGS := "-Ctarget-cpu=x86-64-v3 -Cllvm-args=--cost-kind=throughput -Clinker-plugin-lto -Clink-arg=-fuse-ld=lld"
export CC        := "clang"
export CXX       := "clang++"
export CFLAGS    := `llvm-config --cflags` + " -march=x86-64-v3 -Wall -Wextra -flto"
export CXXFLAGS  := `llvm-config --cxxflags` + " -march=x86-64-v3 -Wall -Wextra -flto"
export LDFLAGS   := `llvm-config --ldflags` + " -fuse-ld=lld -flto"



# Build Release!
@build: _caniuse
	# First let's build the Rust bit.
	cargo build \
		--bin "{{ pkg_id }}" \
		--release \
		--all-features \
		--target-dir "{{ cargo_dir }}"


# Build Debian package!
@build-deb: clean credits build
	# cargo-deb doesn't support target_dir flags yet.
	[ ! -d "{{ justfile_directory() }}/target" ] || rm -rf "{{ justfile_directory() }}/target"
	mv "{{ cargo_dir }}" "{{ justfile_directory() }}/target"

	# Build the deb.
	cargo-deb \
		--no-build \
		--quiet \
		-p {{ pkg_id }} \
		-o "{{ release_dir }}"

	just _fix-chown "{{ release_dir }}"
	mv "{{ justfile_directory() }}/target" "{{ cargo_dir }}"


@clean:
	# Most things go here.
	[ ! -d "{{ cargo_dir }}" ] || rm -rf "{{ cargo_dir }}"

	# But some Cargo apps place shit in subdirectories even if
	# they place *other* shit in the designated target dir. Haha.
	[ ! -d "{{ justfile_directory() }}/target" ] || rm -rf "{{ justfile_directory() }}/target"
	[ ! -d "{{ pkg_dir1 }}/target" ] || rm -rf "{{ pkg_dir1 }}/target"
	[ ! -d "{{ pkg_dir2 }}/target" ] || rm -rf "{{ pkg_dir2 }}/target"

	# Clear caniuse data.
	[ ! -f "{{ caniuse_tmp }}" ] || rm "{{ caniuse_tmp }}"

	cargo update


# Clippy.
@clippy:
	clear
	cargo clippy \
		--release \
		--all-features \
		--target-dir "{{ cargo_dir }}"


# Generate CREDITS.
@credits:
	cargo bashman -m "{{ pkg_dir1 }}/Cargo.toml" -t x86_64-unknown-linux-gnu
	just _fix-chown "{{ justfile_directory() }}/CREDITS.md"


# Build Docs.
@doc:
	# Make the docs.
	cargo rustdoc \
		--release \
		--target-dir "{{ cargo_dir }}"

	# Move the docs and clean up ownership.
	[ ! -d "{{ doc_dir }}" ] || rm -rf "{{ doc_dir }}"
	mv "{{ cargo_dir }}/doc" "{{ justfile_directory() }}"
	just _fix-chown "{{ doc_dir }}"

	exit 0


# Test Run.
@run +ARGS:
	cargo run \
		--bin "{{ pkg_id }}" \
		--all-features \
		--release \
		--target-dir "{{ cargo_dir }}" \
		-- {{ ARGS }}


# Unit Tests!
@test:
	clear

	fyi task "Test (Release)"
	cargo test \
		--all-features \
		--release \
		--target-dir "{{ cargo_dir }}"

	just _test-debug


# Unit Tests (Debug).
_test-debug:
	#!/usr/bin/env bash
	set -e

	unset -v RUSTFLAGS CC CXX CFLAGS CXXFLAGS LDFLAGS

	fyi task "Test (Debug)"
	cargo test \
		--all-features \
		--target-dir "{{ cargo_dir }}"


# Get/Set version.
version:
	#!/usr/bin/env bash

	# Current version.
	_ver1="$( toml get "{{ pkg_dir1 }}/Cargo.toml" package.version | \
		sed 's/"//g' )"

	# Find out if we want to bump it.
	_ver2="$( whiptail --inputbox "Set {{ pkg_name }} version:" --title "Release Version" 0 0 "$_ver1" 3>&1 1>&2 2>&3 )"

	exitstatus=$?
	if [ $exitstatus != 0 ] || [ "$_ver1" = "$_ver2" ]; then
		exit 0
	fi

	fyi success "Setting version to $_ver2."

	# Set the release version!
	just _version "{{ pkg_dir1 }}" "$_ver2"
	just _version "{{ pkg_dir2 }}" "$_ver2"


# Set version for real.
@_version DIR VER:
	[ -f "{{ DIR }}/Cargo.toml" ] || exit 1

	# Set the release version!
	toml set "{{ DIR }}/Cargo.toml" package.version "{{ VER }}" > /tmp/Cargo.toml
	just _fix-chown "/tmp/Cargo.toml"
	mv "/tmp/Cargo.toml" "{{ DIR }}/Cargo.toml"


# Bench Reset.
@_bench-reset:
	[ ! -d "{{ data_dir }}" ] || rm -rf "{{ data_dir }}"
	cp -a "{{ justfile_directory() }}/skel" "{{ data_dir }}"
	just _fix-chown "{{ data_dir }}"


# Refresh Remote Data.
@_caniuse:
	if [ ! -f "{{ caniuse_tmp }}" ]; then \
		wget -q -O "{{ caniuse_tmp }}" "{{ caniuse_url }}"; \
		cat "{{ caniuse_tmp }}" | jq '{agents: .agents}' > "{{ pkg_dir2 }}/skel/data-2.0.json"; \
	fi


# Init dependencies.
@_init:
	# Nothing here just now.

# Fix file/directory permissions.
@_fix-chmod PATH:
	[ ! -e "{{ PATH }}" ] || find "{{ PATH }}" -type f -exec chmod 0644 {} +
	[ ! -e "{{ PATH }}" ] || find "{{ PATH }}" -type d -exec chmod 0755 {} +


# Fix file/directory ownership.
@_fix-chown PATH:
	[ ! -e "{{ PATH }}" ] || chown -R --reference="{{ justfile() }}" "{{ PATH }}"
