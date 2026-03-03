#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
XZ_SRC_DIR="${XZ_SRC_DIR:-$ROOT_DIR/liblzma-sys/xz}"
XZ_BUILD_DIR="${XZ_BUILD_DIR:-$ROOT_DIR/target/xz-cmake-rslink}"
PROFILE="${PROFILE:-release}"
XZ_THREADS="${XZ_THREADS:-yes}"
JOBS="${JOBS:-$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 8)}"
RS_SYS_FEATURES="${RS_SYS_FEATURES:-}"

# Keep Rust-side ABI surface aligned with the xz C build configuration.
if [[ -z "$RS_SYS_FEATURES" && "$XZ_THREADS" != "no" ]]; then
  RS_SYS_FEATURES="parallel"
fi

if [[ ! -d "$XZ_SRC_DIR" ]]; then
  echo "xz source tree not found: $XZ_SRC_DIR" >&2
  exit 1
fi

echo "[1/6] Configure upstream xz C test build (XZ_THREADS=$XZ_THREADS)"
cmake \
  -S "$XZ_SRC_DIR" \
  -B "$XZ_BUILD_DIR" \
  -DBUILD_TESTING=ON \
  -DXZ_THREADS="$XZ_THREADS" \
  >/dev/null

echo "[2/6] Build upstream liblzma once (for test target graph)"
cmake --build "$XZ_BUILD_DIR" --target liblzma -j"$JOBS"

echo "[3/6] Build Rust liblzma static library via liblzma-rs-sys (profile=$PROFILE)"
if [[ -n "$RS_SYS_FEATURES" ]]; then
  cargo build -p liblzma-rs-sys --profile "$PROFILE" --features "$RS_SYS_FEATURES"
else
  cargo build -p liblzma-rs-sys --profile "$PROFILE"
fi

RS_LIBLZMA_A="$ROOT_DIR/target/$PROFILE/libliblzma_rs_sys.a"
if [[ ! -f "$RS_LIBLZMA_A" ]]; then
  echo "Rust static library not found: $RS_LIBLZMA_A" >&2
  exit 1
fi

echo "[4/6] Replace upstream liblzma.a with Rust staticlib"
cp "$RS_LIBLZMA_A" "$XZ_BUILD_DIR/liblzma.a"

echo "[5/6] Build xz + C tests linked against Rust liblzma"
cmake --build "$XZ_BUILD_DIR" -j"$JOBS"

echo "[6/6] Run upstream C tests"
ctest --test-dir "$XZ_BUILD_DIR" --output-on-failure -j"$JOBS"
