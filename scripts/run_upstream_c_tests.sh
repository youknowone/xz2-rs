#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

BACKEND="${BACKEND:-xz-sys}"
PROFILE="${PROFILE:-release}"
SKIP_BUILD="${SKIP_BUILD:-0}"
LANE="${LANE:-baseline}"

BUILD_ENV=(
  env
  -u RUSTFLAGS
  -u CFLAGS
  -u CXXFLAGS
  -u CPPFLAGS
  -u LDFLAGS
  -u CC
  -u CXX
  -u AR
  -u RANLIB
  -u MACOSX_DEPLOYMENT_TARGET
)

CC_ENV=(
  env
  -u CFLAGS
  -u CXXFLAGS
  -u CPPFLAGS
  -u LDFLAGS
  -u CC
  -u CXX
  -u AR
  -u RANLIB
  -u MACOSX_DEPLOYMENT_TARGET
)

if [[ "$BACKEND" != "xz-sys" && "$BACKEND" != "liblzma-sys" ]]; then
  echo "BACKEND must be xz-sys or liblzma-sys" >&2
  exit 2
fi

if [[ "$PROFILE" != "debug" && "$PROFILE" != "release" ]]; then
  echo "PROFILE must be debug or release" >&2
  exit 2
fi

case "$LANE" in
  baseline)
    ;;
  best-effort-thin)
    ;;
  best-effort-fat)
    ;;
  *)
    echo "LANE must be baseline, best-effort-thin, or best-effort-fat" >&2
    exit 2
    ;;
esac

TESTS=("$@")
if [[ ${#TESTS[@]} -eq 0 ]]; then
  TESTS=(
    test_bcj_exact_size
    test_block_header
    test_check
    test_filter_flags
    test_filter_str
    test_hardware
    test_index
    test_index_hash
    test_lzip_decoder
    test_memlimit
    test_stream_flags
    test_vli
  )
fi

BUILD_ARGS=()
if [[ "$PROFILE" == "release" ]]; then
  BUILD_ARGS+=(--release)
fi

TARGET_DIR="target/upstream-c-tests/build/${LANE}/${PROFILE}/${BACKEND}"
RESULT_DIR="target/upstream-c-tests/${LANE}/${PROFILE}/${BACKEND}"
mkdir -p "$RESULT_DIR"

RUST_BACKEND_FLAGS="<unset>"
XZ_BUILD_ENV=("${BUILD_ENV[@]}")
if [[ "$LANE" == "best-effort-thin" ]]; then
  RUST_BACKEND_FLAGS="-Clto=thin -Ccodegen-units=1 -Cembed-bitcode=yes"
  XZ_BUILD_ENV+=(RUSTFLAGS="$RUST_BACKEND_FLAGS")
elif [[ "$LANE" == "best-effort-fat" ]]; then
  RUST_BACKEND_FLAGS="-Clto=fat -Ccodegen-units=1 -Cembed-bitcode=yes"
  XZ_BUILD_ENV+=(RUSTFLAGS="$RUST_BACKEND_FLAGS")
fi

LIBLZMA_FEATURES=(static)
if [[ "$LANE" == "best-effort-thin" ]]; then
  LIBLZMA_FEATURES+=(thin-lto)
elif [[ "$LANE" == "best-effort-fat" ]]; then
  LIBLZMA_FEATURES+=(fat-lto)
fi

case "$(uname -s)" in
  Darwin) DEFAULT_NATIVE_LIBS="-liconv -lSystem -lc -lm" ;;
  Linux) DEFAULT_NATIVE_LIBS="-lc -lm" ;;
  *) DEFAULT_NATIVE_LIBS="-lc -lm" ;;
esac

if [[ "$BACKEND" == "xz-sys" ]]; then
  if [[ "$SKIP_BUILD" != "1" ]]; then
    NATIVE_LIBS="$(
      "${XZ_BUILD_ENV[@]}" CARGO_TARGET_DIR="$TARGET_DIR" cargo rustc -p xz-sys "${BUILD_ARGS[@]}" --crate-type staticlib -- --print native-static-libs 2>&1 \
        | sed -n 's/^note: native-static-libs: //p' \
        | tail -n 1
    )"
  else
    NATIVE_LIBS="$DEFAULT_NATIVE_LIBS"
  fi

  if [[ "$PROFILE" == "release" ]]; then
    STATICLIB="$(ls -t "${TARGET_DIR}/release/deps"/libxz_sys-*.a 2>/dev/null | head -n 1)"
  else
    STATICLIB="$(ls -t "${TARGET_DIR}/debug/deps"/libxz_sys-*.a 2>/dev/null | head -n 1)"
  fi
else
  NATIVE_LIBS="$DEFAULT_NATIVE_LIBS"

  if [[ "$SKIP_BUILD" != "1" ]]; then
    "${BUILD_ENV[@]}" CARGO_TARGET_DIR="$TARGET_DIR" cargo build -p liblzma-sys --features "$(IFS=,; echo "${LIBLZMA_FEATURES[*]}")" "${BUILD_ARGS[@]}" >/dev/null
  fi

  if [[ "$PROFILE" == "release" ]]; then
    STATICLIB="$(ls -t "${TARGET_DIR}/release/build"/liblzma-sys-*/out/liblzma.a 2>/dev/null | head -n 1)"
  else
    STATICLIB="$(ls -t "${TARGET_DIR}/debug/build"/liblzma-sys-*/out/liblzma.a 2>/dev/null | head -n 1)"
  fi
fi

if [[ -z "${STATICLIB:-}" ]]; then
  echo "failed to locate built static library for ${BACKEND} (${PROFILE})" >&2
  exit 1
fi

OUT_DIR="${RESULT_DIR}/bin"
mkdir -p "$OUT_DIR"

CFLAGS=(
  -std=c99
  -O2
  -DNDEBUG
  -DHAVE_CONFIG_H=1
  -DHAVE_INTTYPES_H=1
  -DHAVE_STDINT_H=1
  -DHAVE_STDBOOL_H=1
  -DHAVE_STRING_H=1
  -DHAVE_ENCODERS=1
  -DHAVE_DECODERS=1
  -DHAVE_CHECK_CRC32=1
  -DHAVE_CHECK_CRC64=1
  -DHAVE_CHECK_SHA256=1
  -DLZMA_API_STATIC
  -Iliblzma-sys
  -Iliblzma-sys/xz/src/common
  -Iliblzma-sys/xz/src/liblzma/api
  -Iliblzma-sys/xz/src/liblzma
)

read -r -a LINK_LIBS <<<"$NATIVE_LIBS"

{
  echo "lane=${LANE}"
  echo "backend=${BACKEND}"
  echo "profile=${PROFILE}"
  echo "target_dir=${TARGET_DIR}"
  echo "staticlib=${STATICLIB}"
  echo "native_libs=${NATIVE_LIBS}"
  if [[ "$BACKEND" == "liblzma-sys" ]]; then
    echo "features=$(IFS=,; echo "${LIBLZMA_FEATURES[*]}")"
  else
    echo "features=default"
  fi
  if [[ "$BACKEND" == "xz-sys" ]]; then
    echo "rustflags=${RUST_BACKEND_FLAGS}"
  else
    echo "rustflags=<unset>"
  fi
  echo "harness_cflags=${CFLAGS[*]}"
} >"${RESULT_DIR}/build-info.txt"

failures=0
skips=0

for test_name in "${TESTS[@]}"; do
  src="liblzma-sys/xz/tests/${test_name}.c"
  exe="${OUT_DIR}/${test_name}"

  if [[ ! -f "$src" ]]; then
    echo "missing upstream test source: $src" >&2
    failures=$((failures + 1))
    continue
  fi

  echo "== compile ${test_name} =="
  if ! "${CC_ENV[@]}" cc "${CFLAGS[@]}" -o "$exe" "$src" "$STATICLIB" "${LINK_LIBS[@]}"; then
    failures=$((failures + 1))
    continue
  fi

  echo "== run ${test_name} =="
  set +e
  srcdir="$ROOT_DIR/liblzma-sys/xz/tests" "$exe"
  status=$?
  set -e

  case "$status" in
    0)
      ;;
    77)
      skips=$((skips + 1))
      ;;
    *)
      failures=$((failures + 1))
      ;;
  esac
done

echo
echo "compiled: ${#TESTS[@]}"
echo "skipped: ${skips}"
echo "failed: ${failures}"

if [[ "$failures" -ne 0 ]]; then
  exit 1
fi
