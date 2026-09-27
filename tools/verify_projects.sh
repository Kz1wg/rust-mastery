#!/usr/bin/env bash
# projects/solutions/<name>/ の .rs を、projects/<name>/ の同じ相対パスへ一時的に重ねて
# cargo test（既定）/ fmt / clippy を検証する。終了時には必ず骨組みへ復元する。
# solutions/ に対応が無いもの（final_* の参考実装）は、そのまま検証する。
#
# 使い方:
#   ./tools/verify_projects.sh              # cargo test のみ
#   ./tools/verify_projects.sh --fmt --clippy
#
# tools/verify_solutions.sh（exercises/ 用）と同じ考え方。projects/ は独立した
# workspace なので、別のスクリプトにしている。

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT/projects"

RUN_CLIPPY=0
RUN_FMT=0
for arg in "$@"; do
  case "$arg" in
    --clippy) RUN_CLIPPY=1 ;;
    --fmt) RUN_FMT=1 ;;
    *) echo "unknown option: $arg" >&2; exit 2 ;;
  esac
done

declare -a BACKUPS=()
declare -a RESTORE_TARGETS=()

cleanup() {
  local status=$?
  for i in "${!RESTORE_TARGETS[@]}"; do
    cp "${BACKUPS[$i]}" "${RESTORE_TARGETS[$i]}"
    rm -f "${BACKUPS[$i]}"
  done
  exit "$status"
}
trap cleanup EXIT INT TERM

overlay() {
  local target="$1" source="$2" backup
  backup="$(mktemp)"
  cp "$target" "$backup"
  BACKUPS+=("$backup")
  RESTORE_TARGETS+=("$target")
  cp "$source" "$target"
}

PASSED=0
FAILED=0
declare -a FAILED_NAMES=()

for dir in p*/ final_*/; do
  [ -d "$dir" ] || continue
  name="$(basename "$dir")"
  if [ -d "solutions/${name}" ]; then
    echo "== $name =="
    while IFS= read -r sol; do
      rel="${sol#solutions/${name}/}"
      [ -f "${dir}${rel}" ] && overlay "${dir}${rel}" "$sol"
    done < <(find "solutions/${name}" -name '*.rs')
  else
    # 解答を重ねない＝そのままで完成しているコード（Final Project の参考実装）
    echo "== $name（参考実装。そのまま検証）=="
  fi

  ok=1
  cargo test --quiet -p "$name" || ok=0
  if [ "$RUN_FMT" -eq 1 ]; then cargo fmt -p "$name" --check || ok=0; fi
  if [ "$RUN_CLIPPY" -eq 1 ]; then cargo clippy -p "$name" -- -D warnings || ok=0; fi

  if [ "$ok" -eq 1 ]; then
    echo "  OK"; PASSED=$((PASSED + 1))
  else
    echo "  NG"; FAILED=$((FAILED + 1)); FAILED_NAMES+=("$name")
  fi
done

echo
echo "Passed: $PASSED  Failed: $FAILED"
if [ "$FAILED" -gt 0 ]; then
  echo "Failed projects: ${FAILED_NAMES[*]}"
  exit 1
fi
