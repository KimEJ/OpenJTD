#!/usr/bin/env bash
set -euo pipefail

readonly repository_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
readonly inventory_path="$repository_root/rjtd/licenses/wasm/DEPENDENCIES.lock.tsv"
readonly notices_path="$repository_root/rjtd/licenses/wasm/THIRD_PARTY_NOTICES.txt"
readonly cargo_lock_path="$repository_root/rjtd/Cargo.lock"
readonly source_cutoff="2026-07-16T02:11:35Z"
readonly registry_source="registry+https://github.com/rust-lang/crates.io-index"
readonly inventory_header=$'package\tversion\tcargo_lock_checksum\tdeclared_license\tsource_url\tarchive_root_text\tarchive_root_text_sha256'

temporary_directory="$(mktemp -d)"
trap 'rm -rf -- "$temporary_directory"' EXIT

die() {
  printf '%s\n' "wasm distribution notices: $*" >&2
  exit 1
}

checksum_of() {
  shasum -a 256 "$1" | awk '{print $1}'
}

archive_for() {
  local package="$1"
  local version="$2"
  local expected_checksum="$3"
  local candidate

  while IFS= read -r candidate; do
    if [[ "$(checksum_of "$candidate")" == "$expected_checksum" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done < <(find "${CARGO_HOME:-$HOME/.cargo}/registry/cache" -type f -name "$package-$version.crate" -print 2>/dev/null)

  die "no cached archive with the locked checksum for $package $version"
}

validate_inventory_format() {
  [[ -f "$inventory_path" ]] || die "missing inventory"
  [[ "$(sed -n '1p' "$inventory_path")" == "$inventory_header" ]] || die "unexpected inventory header"

  local record_count
  record_count="$(awk -F '\t' '
    NR == 1 { next }
    {
      if (NF != 7 || $0 ~ /\r$/ || $1 !~ /^[a-z0-9][a-z0-9_-]*$/ || $2 !~ /^[0-9][0-9A-Za-z.+-]*$/ || $3 !~ /^[0-9a-f]{64}$/ || $4 == "" || $5 != "https://crates.io/crates/" $1 "/" $2 || $6 !~ /^(LICENSE|NOTICE)/ || $6 ~ /\// || $7 !~ /^[0-9a-f]{64}$/) { invalid = 1; next }
      seen_row[$0]++
      seen_text[$1 FS $2 FS $6]++
      package_key = $1 FS $2
      package_details = $3 FS $4 FS $5
      if ((package_key in details) && details[package_key] != package_details) { invalid = 1 }
      details[package_key] = package_details
      records += 1
    }
    END {
      for (row in seen_row) { if (seen_row[row] != 1) { invalid = 1 } }
      for (text in seen_text) { if (seen_text[text] != 1) { invalid = 1 } }
      if (invalid) { exit 1 }
      print records
    }
  ' "$inventory_path")" || die "malformed, duplicate, or inconsistent inventory record"

  [[ "$record_count" == "33" ]] || die "expected 33 archive-root records, found $record_count"

  awk -F '\t' 'NR > 1 { key = $1 FS $2; if (!seen[key]++) { print $1 FS $2 FS $3 FS $4 FS $5 } }' "$inventory_path" | LC_ALL=C sort > "$temporary_directory/inventory-packages.tsv"
  [[ "$(wc -l < "$temporary_directory/inventory-packages.tsv" | tr -d ' ')" == "17" ]] || die "expected 17 locked packages"
}

validate_closure() {
  (
    cd "$repository_root/rjtd"
    cargo tree -p rjtd-wasm --target wasm32-unknown-unknown --locked --offline \
      --edges no-dev,no-build,no-proc-macro --prefix none --format '{p}'
  ) > "$temporary_directory/cargo-tree.tsv"

  awk '
    {
      split($0, columns, /\\t/)
      package = columns[1]
      sub(/ \(\*\)$/, "", package)
      if (package ~ /^rjtd-[^ ]+ v[^ ]+ \(/) { next }
      if (package !~ /^[^ ]+ v[^ ]+$/) {
        print "unexpected cargo tree package: " package > "/dev/stderr"
        invalid = 1
        next
      }
      split(package, fields, " ")
      sub(/^v/, "", fields[2])
      print fields[1] "\t" fields[2]
    }
    END { if (invalid) { exit 1 } }
  ' "$temporary_directory/cargo-tree.tsv" | LC_ALL=C sort -u > "$temporary_directory/closure.tsv" || die "could not normalize the locked wasm closure"

  cut -f1,2 "$temporary_directory/inventory-packages.tsv" | LC_ALL=C sort > "$temporary_directory/inventory-closure.tsv"
  cmp -s "$temporary_directory/closure.tsv" "$temporary_directory/inventory-closure.tsv" || die "inventory does not match the locked wasm closure"
}

validate_lock_checksums() {
  awk '
    function emit() {
      if (name != "" && source == "registry+https://github.com/rust-lang/crates.io-index") {
        if (version == "" || checksum == "") { invalid = 1 }
        else { print name "\t" version "\t" checksum }
      }
    }
    /^\[\[package\]\]$/ { emit(); name = version = source = checksum = ""; next }
    $1 == "name" { name = $3; gsub(/"/, "", name); next }
    $1 == "version" { version = $3; gsub(/"/, "", version); next }
    $1 == "source" { source = $3; gsub(/"/, "", source); next }
    $1 == "checksum" { checksum = $3; gsub(/"/, "", checksum); next }
    END { emit(); if (invalid) { exit 1 } }
  ' "$cargo_lock_path" > "$temporary_directory/lock-checksums.tsv" || die "could not read registry checksums from Cargo.lock"

  local package version expected_checksum declared_license source_url observed_checksum
  while IFS=$'\t' read -r package version expected_checksum declared_license source_url; do
    observed_checksum="$(awk -F '\t' -v package="$package" -v version="$version" '$1 == package && $2 == version { print $3 }' "$temporary_directory/lock-checksums.tsv")"
    [[ "$observed_checksum" == "$expected_checksum" ]] || die "Cargo.lock checksum mismatch for $package $version"
  done < "$temporary_directory/inventory-packages.tsv"
}

validate_archives() {
  local package version expected_checksum declared_license source_url archive expected_texts actual_texts
  while IFS=$'\t' read -r package version expected_checksum declared_license source_url; do
    archive="$(archive_for "$package" "$version" "$expected_checksum")"
    expected_texts="$temporary_directory/$package-$version-expected.txt"
    actual_texts="$temporary_directory/$package-$version-actual.txt"
    awk -F '\t' -v package="$package" -v version="$version" 'NR > 1 && $1 == package && $2 == version { print $6 }' "$inventory_path" | LC_ALL=C sort > "$expected_texts"
    tar -tzf "$archive" | awk -F/ -v root="$package-$version" 'NF == 2 && $1 == root && $2 ~ /^(LICENSE|NOTICE)/ { print $2 }' | LC_ALL=C sort > "$actual_texts"
    cmp -s "$expected_texts" "$actual_texts" || die "archive-root text inventory mismatch for $package $version"
  done < "$temporary_directory/inventory-packages.tsv"

  local archive_root_text archive_root_text_sha256 observed_text_sha256
  while IFS=$'\t' read -r package version expected_checksum declared_license source_url archive_root_text archive_root_text_sha256; do
    archive="$(archive_for "$package" "$version" "$expected_checksum")"
    observed_text_sha256="$(tar -xOf "$archive" "$package-$version/$archive_root_text" | shasum -a 256 | awk '{print $1}')"
    [[ "$observed_text_sha256" == "$archive_root_text_sha256" ]] || die "archive-root text hash mismatch for $package $version $archive_root_text"
  done < <(awk 'NR > 1 { print }' "$inventory_path")
}

validate_declared_licenses() {
  local package version expected_checksum declared_license source_url archive observed_license
  while IFS=$'\t' read -r package version expected_checksum declared_license source_url; do
    archive="$(archive_for "$package" "$version" "$expected_checksum")"
    observed_license="$(
      tar -xOf "$archive" "$package-$version/Cargo.toml" \
        | sed -nE 's/^license[[:space:]]*=[[:space:]]*"([^"]*)".*/\1/p' \
        | head -n 1
    )"
    [[ -n "$observed_license" ]] || die "archive Cargo.toml has no declared license for $package $version"
    [[ "$observed_license" == "$declared_license" ]] || die "archive Cargo.toml license mismatch for $package $version"
  done < "$temporary_directory/inventory-packages.tsv"
}

render_notices() {
  local cargo_lock_checksum package version expected_checksum declared_license source_url archive_root_text archive_root_text_sha256 archive current_package=""
  cargo_lock_checksum="$(checksum_of "$cargo_lock_path")"

  printf '%s\n\n' 'OpenJTD WASM third-party notices'
  printf 'Source cutoff: %s\n' "$source_cutoff"
  printf 'Cargo.lock SHA-256: %s\n' "$cargo_lock_checksum"
  printf '%s\n\n' 'Source scope: locked rjtd-wasm normal-runtime dependency closure for wasm32-unknown-unknown, excluding development, build, and procedural-macro edges.'
  printf '%s\n\n' 'Package inventory'
  while IFS=$'\t' read -r package version expected_checksum declared_license source_url; do
    printf '%s %s — %s\n' "$package" "$version" "$declared_license"
  done < "$temporary_directory/inventory-packages.tsv"

  printf '\n%s\n' 'Retained archive-root source texts'
  while IFS=$'\t' read -r package version expected_checksum declared_license source_url archive_root_text archive_root_text_sha256; do
    if [[ "$current_package" != "$package $version" ]]; then
      current_package="$package $version"
      printf '\n## %s\n' "$current_package"
      printf 'Source: %s\n' "$source_url"
      printf 'Archive SHA-256: %s\n' "$expected_checksum"
    fi
    printf '\n### %s\n\n' "$archive_root_text"
    archive="$(archive_for "$package" "$version" "$expected_checksum")"
    tar -xOf "$archive" "$package-$version/$archive_root_text"
  done < <(awk 'NR > 1 { print }' "$inventory_path" | LC_ALL=C sort)
}

verify() {
  validate_inventory_format
  validate_closure
  validate_lock_checksums
  validate_archives
  validate_declared_licenses
}

case "${1:-}" in
  '')
    verify
    render_notices > "$temporary_directory/expected-notices.txt"
    [[ -s "$notices_path" ]] || die "missing or empty notice bundle"
    cmp -s "$temporary_directory/expected-notices.txt" "$notices_path" || die "notice bundle is incomplete or non-deterministic"
    printf '%s\n' 'WASM distribution notices verified.'
    ;;
  --render)
    verify
    render_notices
    ;;
  --write)
    verify
    render_notices > "$temporary_directory/THIRD_PARTY_NOTICES.txt"
    mv "$temporary_directory/THIRD_PARTY_NOTICES.txt" "$notices_path"
    ;;
  *)
    die "usage: $0 [--render|--write]"
    ;;
esac
