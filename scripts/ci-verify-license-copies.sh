#!/usr/bin/env bash
set -euo pipefail

readonly repository_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
readonly license_copies=(
  rjtd/crates/rjtd-core/LICENSE
  rjtd/crates/rjtd-model/LICENSE
  rjtd/crates/rjtd-export/LICENSE
  rjtd/crates/rjtd-cli/LICENSE
  rjtd/crates/rjtd-wasm/LICENSE
)

for license_copy in "${license_copies[@]}"; do
  test -f "$repository_root/$license_copy"
  cmp --silent "$repository_root/LICENSE" "$repository_root/$license_copy"
done
