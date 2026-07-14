#!/usr/bin/env bash
set -euo pipefail

readonly repository_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
readonly public_crates=(rjtd-core rjtd-model rjtd-export rjtd-cli rjtd-wasm)
readonly required_package_files=(Cargo.toml README.md LICENSE)

cd "$repository_root/rjtd"
package_list="$(mktemp)"
trap 'rm -f "$package_list"' EXIT

for crate in "${public_crates[@]}"; do
  crate_dir="crates/$crate"

  grep --fixed-strings --line-regexp 'publish = ["crates-io"]' "$crate_dir/Cargo.toml" >/dev/null
  cargo package --package "$crate" --allow-dirty --locked --no-verify --list > "$package_list"

  for file in "${required_package_files[@]}"; do
    grep --fixed-strings --line-regexp "$file" "$package_list" >/dev/null
  done

  grep --extended-regexp '^src/.+\.rs$' "$package_list" >/dev/null
done

grep --fixed-strings --line-regexp 'publish = false' crates/rjtd-testkit/Cargo.toml >/dev/null
