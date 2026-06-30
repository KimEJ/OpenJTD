#!/usr/bin/env sh
set -eu

repo_root="$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)"
sample_dir="${RJTD_LOCAL_SAMPLE_DIR:-$repo_root/rjtd-testdata/local-samples}"
output_dir="${RJTD_PDF_OUTPUT_DIR:-$repo_root/openjtd-samples/pdf-output}"

if [ ! -d "$sample_dir" ]; then
  echo "local sample directory not found: $sample_dir" >&2
  exit 1
fi

mkdir -p "$output_dir"

cd "$repo_root"
cargo build --manifest-path "$repo_root/rjtd/Cargo.toml" -p rjtd-cli

if [ -n "${CARGO_TARGET_DIR:-}" ]; then
  case "$CARGO_TARGET_DIR" in
    /*) target_dir="$CARGO_TARGET_DIR" ;;
    *) target_dir="$repo_root/$CARGO_TARGET_DIR" ;;
  esac
else
  target_dir="$repo_root/rjtd/target"
fi
bin="$target_dir/debug/rjtd"

if [ ! -x "$bin" ]; then
  echo "rjtd binary not found after build: $bin" >&2
  exit 1
fi

list_file="$(mktemp "${TMPDIR:-/tmp}/rjtd-pdf-samples.XXXXXX")"
trap 'rm -f "$list_file"' EXIT

find "$sample_dir" -maxdepth 1 -type f \( -name '*.jtd' -o -name '*.jtt' -o -name '*.jttc' \) | sort > "$list_file"

count=0
while IFS= read -r sample_path; do
  [ -n "$sample_path" ] || continue
  stem="$(basename "$sample_path")"
  stem="${stem%.*}"
  output_path="$output_dir/$stem.pdf"
  echo "render $sample_path -> $output_path"
  "$bin" export "$sample_path" --format pdf -o "$output_path"
  count=$((count + 1))
done < "$list_file"

echo "regenerated $count PDF artifact(s) under $output_dir"
