#!/usr/bin/env bash
set -euo pipefail

readonly version='0.13.1'
readonly archive="wasm-pack-v${version}-x86_64-unknown-linux-musl.tar.gz"
readonly sha256='c539d91ccab2591a7e975bcf82c82e1911b03335c80aa83d67ad25ed2ad06539'
readonly url="https://github.com/wasm-bindgen/wasm-pack/releases/download/v${version}/${archive}"

: "${RUNNER_TEMP:?RUNNER_TEMP must be set}"
archive_path="${RUNNER_TEMP}/${archive}"
install_dir="$(mktemp -d "${RUNNER_TEMP}/wasm-pack.XXXXXX")"

curl --fail --location --proto '=https' --tlsv1.2 --retry 3 --output "$archive_path" "$url"
printf '%s  %s\n' "$sha256" "$archive_path" | sha256sum --check --status
tar --extract --gzip --file "$archive_path" --directory "$install_dir" --strip-components=1
test -x "${install_dir}/wasm-pack"

printf '%s\n' "$install_dir"
