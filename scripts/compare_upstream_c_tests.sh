#!/usr/bin/env bash
set -euo pipefail

RUNS=5
WARMUP=1
PROFILE=release
RESULTS_DIR="target/perf-results"
LANE=baseline

while [[ $# -gt 0 ]]; do
  case "$1" in
    --lane)
      LANE="$2"
      shift 2
      ;;
    --runs)
      RUNS="$2"
      shift 2
      ;;
    --warmup)
      WARMUP="$2"
      shift 2
      ;;
    --profile)
      PROFILE="$2"
      shift 2
      ;;
    --results-dir)
      RESULTS_DIR="$2"
      shift 2
      ;;
    --help|-h)
      cat <<'EOF'
Usage: scripts/compare_upstream_c_tests.sh [options] [test_name...]

Compare upstream vendored liblzma C tests linked against:
  - xz-sys
  - liblzma-sys (vendored C liblzma)

Options:
  --lane <name>      baseline|best-effort-thin|best-effort-fat (default: baseline)
  --runs <n>          Number of measured runs per command (default: 5)
  --warmup <n>        Warmup runs per command (default: 1)
  --profile <mode>    debug|release (default: release)
  --results-dir <dir> Where to write hyperfine reports
EOF
      exit 0
      ;;
    *)
      break
      ;;
  esac
done

command -v hyperfine >/dev/null 2>&1 || {
  echo "hyperfine is required for compare_upstream_c_tests.sh" >&2
  exit 1
}

mkdir -p "$RESULTS_DIR"

TESTS=("$@")

PROFILE="$PROFILE" LANE="$LANE" BACKEND=xz-sys scripts/run_upstream_c_tests.sh "${TESTS[@]}" >/dev/null
PROFILE="$PROFILE" LANE="$LANE" BACKEND=liblzma-sys scripts/run_upstream_c_tests.sh "${TESTS[@]}" >/dev/null

XZ_CMD="env PROFILE=$PROFILE LANE=$LANE BACKEND=xz-sys SKIP_BUILD=1 scripts/run_upstream_c_tests.sh"
C_CMD="env PROFILE=$PROFILE LANE=$LANE BACKEND=liblzma-sys SKIP_BUILD=1 scripts/run_upstream_c_tests.sh"

if [[ ${#TESTS[@]} -gt 0 ]]; then
  for test_name in "${TESTS[@]}"; do
    XZ_CMD+=" ${test_name}"
    C_CMD+=" ${test_name}"
  done
  SUFFIX="$(printf '%s-' "${TESTS[@]}")"
  SUFFIX="${SUFFIX%-}"
else
  SUFFIX="all"
fi

REPORT_STEM="$RESULTS_DIR/upstream-c-tests-${SUFFIX}-${LANE}"

hyperfine \
  --runs "$RUNS" \
  --warmup "$WARMUP" \
  --export-json "${REPORT_STEM}.json" \
  --export-markdown "${REPORT_STEM}.md" \
  --command-name xz-sys \
  "$XZ_CMD" \
  --command-name liblzma-sys \
  "$C_CMD"

cat >>"${REPORT_STEM}.md" <<EOF

## Build Lane

- lane: \`${LANE}\`
- profile: \`${PROFILE}\`
- xz-sys build info: [target/upstream-c-tests/${LANE}/${PROFILE}/xz-sys/build-info.txt](../upstream-c-tests/${LANE}/${PROFILE}/xz-sys/build-info.txt)
- liblzma-sys build info: [target/upstream-c-tests/${LANE}/${PROFILE}/liblzma-sys/build-info.txt](../upstream-c-tests/${LANE}/${PROFILE}/liblzma-sys/build-info.txt)
EOF

cat <<EOF
Reports written to:
  ${REPORT_STEM}.json
  ${REPORT_STEM}.md
EOF
