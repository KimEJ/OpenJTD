#!/usr/bin/env bash

set -euo pipefail

readonly release_version="0.0.1"
readonly registry_api="https://crates.io/api/v1/crates"
readonly registry_user_agent="OpenJTD-release-preflight/${release_version}"

allow_dirty=false
package=""
declare -a release_dependencies=()

die() {
    printf 'release preflight: %s\n' "$*" >&2
    exit 1
}

usage() {
    cat <<'EOF'
Usage: scripts/release-preflight.sh --package <package> [--allow-dirty]

Runs a publish-safe, credential-free crates.io preflight for exactly one
OpenJTD 0.0.1 public crate. It never performs a real upload.

Options:
  -p, --package <package>  One of rjtd-core, rjtd-model, rjtd-export,
                           rjtd-wasm, or rjtd-cli.
      --allow-dirty        Local candidate inspection only; never release approval.
  -h, --help               Show this help.
EOF
}

while (($# > 0)); do
    case "$1" in
        -p|--package)
            (($# >= 2)) || die "missing value for $1"
            package="$2"
            shift 2
            ;;
        --allow-dirty)
            allow_dirty=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            usage >&2
            die "unknown argument: $1"
            ;;
    esac
done

[[ -n "$package" ]] || {
    usage >&2
    die "--package is required"
}

case "$package" in
    rjtd-core)
        release_dependencies=()
        ;;
    rjtd-model)
        release_dependencies=(rjtd-core)
        ;;
    rjtd-export)
        release_dependencies=(rjtd-core rjtd-model)
        ;;
    rjtd-wasm)
        release_dependencies=(rjtd-core rjtd-model)
        ;;
    rjtd-cli)
        release_dependencies=(rjtd-core rjtd-model rjtd-export)
        ;;
    rjtd-testkit)
        die "rjtd-testkit is internal and must never be preflighted or published"
        ;;
    *)
        die "unsupported package: $package"
        ;;
esac

for tool in cargo curl git grep mktemp mkdir rm tee dirname cat; do
    command -v "$tool" >/dev/null 2>&1 || die "required tool is unavailable: $tool"
done

script_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(git -C "$script_dir/.." rev-parse --show-toplevel)"
workspace="$repo_root/rjtd"
workspace_manifest="$workspace/Cargo.toml"
package_manifest="$workspace/crates/$package/Cargo.toml"
testkit_manifest="$workspace/crates/rjtd-testkit/Cargo.toml"

[[ -f "$workspace_manifest" ]] || die "missing workspace manifest: $workspace_manifest"
[[ -f "$package_manifest" ]] || die "missing package manifest: $package_manifest"
[[ -f "$testkit_manifest" ]] || die "missing testkit manifest: $testkit_manifest"

if ! grep -Eq '^[[:space:]]*publish[[:space:]]*=[[:space:]]*\[[[:space:]]*"crates-io"[[:space:]]*\][[:space:]]*(#.*)?$' "$package_manifest"; then
    die "$package is not explicitly limited to crates-io"
fi

if ! grep -Eq '^[[:space:]]*publish[[:space:]]*=[[:space:]]*false[[:space:]]*(#.*)?$' "$testkit_manifest"; then
    die "rjtd-testkit must retain publish = false"
fi

if [[ "$allow_dirty" == false ]]; then
    [[ -z "$(git -C "$repo_root" status --porcelain=v1 --untracked-files=all)" ]] || die "repository is dirty; commit or discard changes before release preflight"
else
    printf '%s\n' 'warning: --allow-dirty is a local candidate check, not release approval' >&2
fi

temp_root="$(mktemp -d "${TMPDIR:-/tmp}/rjtd-release-preflight.XXXXXX")"
trap 'rm -rf "$temp_root"' EXIT
mkdir -p "$temp_root/cargo-home" "$temp_root/target"

export CARGO_HOME="$temp_root/cargo-home"
export CARGO_TARGET_DIR="$temp_root/target"
unset CARGO_REGISTRY_TOKEN
unset CARGO_REGISTRIES_CRATES_IO_TOKEN

registry_status() {
    local url="$1"
    curl --silent --show-error --location --connect-timeout 10 --max-time 30 \
        --user-agent "$registry_user_agent" --output /dev/null --write-out '%{http_code}' "$url"
}

assert_unallocated_name() {
    local status
    status="$(registry_status "$registry_api/$package")" || die "could not query crates.io for $package"
    case "$status" in
        404)
            printf 'name check: %s is unallocated at crates.io (point-in-time only)\n' "$package"
            ;;
        200)
            die "$package is already allocated on crates.io; stop before publishing"
            ;;
        *)
            die "unexpected crates.io response for $package: HTTP $status"
            ;;
    esac
}

assert_indexed_dependency() {
    local dependency="$1"
    local status
    status="$(registry_status "$registry_api/$dependency/$release_version")" || die "could not query crates.io for $dependency $release_version"
    case "$status" in
        200)
            printf 'dependency check: %s %s is indexed\n' "$dependency" "$release_version"
            ;;
        404)
            die "$package requires $dependency $release_version in crates.io before its dry-run; wait for index visibility"
            ;;
        *)
            die "unexpected crates.io response for $dependency $release_version: HTTP $status"
            ;;
    esac
}

package_id="$(cargo pkgid --locked -p "$package" --manifest-path "$workspace_manifest")"
[[ "$package_id" == *"#$release_version" ]] || die "expected $package@$release_version, got $package_id"

assert_unallocated_name
if [[ "$package" != "rjtd-core" ]]; then
    for dependency in "${release_dependencies[@]}"; do
        assert_indexed_dependency "$dependency"
    done
fi

package_args=(--locked -p "$package" --manifest-path "$workspace_manifest")
if [[ "$allow_dirty" == true ]]; then
    package_args+=(--allow-dirty)
fi

contents_file="$temp_root/$package.contents"
printf 'package contents for %s:\n' "$package"
cargo package --list "${package_args[@]}" | tee "$contents_file"

for required_file in Cargo.toml LICENSE README.md; do
    grep -Fxq "$required_file" "$contents_file" || die "$package archive omits required file: $required_file"
done
grep -Eq '^src/.+\.rs$' "$contents_file" || die "$package archive contains no Rust source"

printf 'running cargo publish --dry-run for %s@%s\n' "$package" "$release_version"
cargo publish --dry-run "${package_args[@]}"
printf 'release preflight passed: %s@%s; no upload was performed\n' "$package" "$release_version"
