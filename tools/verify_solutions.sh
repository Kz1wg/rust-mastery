#!/usr/bin/env bash
# solutions/ を対応する exercises/ の src/lib.rs に一時的に重ねて
# cargo test（既定）/ clippy / fmt を検証する。
#
# 骨組み（todo!()）のままの exercises/ に対して
# `cargo fmt` / `cargo clippy` を直接走らせると、未使用引数の警告で
# 失敗する（ARCHITECTURE.md §6 を参照）。これは意図した挙動であり、
# 「解答なら通る」ことを確認するのがこのスクリプトの役目。
#
# 使い方:
#   ./tools/verify_solutions.sh              # cargo test のみ
#   ./tools/verify_solutions.sh --clippy      # test に加えて clippy も
#   ./tools/verify_solutions.sh --fmt         # test に加えて fmt --check も
#   ./tools/verify_solutions.sh --clippy --fmt
#
# どの exercises/<name>/src/lib.rs も、このスクリプトの終了時には
# 必ず元の骨組みに戻す（途中で失敗しても trap で復元する）。

set -u

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

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
  local exercise_lib="$1"
  local solution_lib="$2"
  local backup
  backup="$(mktemp)"
  cp "$exercise_lib" "$backup"
  BACKUPS+=("$backup")
  RESTORE_TARGETS+=("$exercise_lib")
  cp "$solution_lib" "$exercise_lib"
}

FAILED=0
PASSED=0
declare -a FAILED_NAMES=()

# --- 入れ子 workspace（ex012, ex048 など。ディレクトリ内で cargo を動かす）---
for dir in exercises/*/; do
  name="$(basename "$dir")"
  grep -q "^\[workspace\]" "${dir}Cargo.toml" 2>/dev/null || continue
  [ -d "solutions/${name}" ] || { echo "== $name: solutions/ がありません（スキップ）=="; continue; }

  echo "== $name (nested workspace) =="
  while IFS= read -r sol; do
    rel="${sol#solutions/${name}/}"
    [ -f "${dir}${rel}" ] && overlay "${dir}${rel}" "$sol"
  done < <(find "solutions/${name}" -name '*.rs')

  ok=1
  ( cd "$dir" && cargo test --quiet ) || ok=0
  if [ "$RUN_FMT" -eq 1 ]; then
    ( cd "$dir" && cargo fmt --all --check ) || ok=0
  fi
  if [ "$RUN_CLIPPY" -eq 1 ]; then
    ( cd "$dir" && cargo clippy --workspace -- -D warnings ) || ok=0
  fi

  if [ "$ok" -eq 1 ]; then
    echo "  OK"
    PASSED=$((PASSED + 1))
  else
    echo "  NG"
    FAILED=$((FAILED + 1))
    FAILED_NAMES+=("$name")
  fi
done

# --- ルートworkspaceの演習（exercises/exNNN_.../src/lib.rs 1ファイルのみのもの）---
for dir in exercises/*/; do
  name="$(basename "$dir")"
  # 入れ子 workspace は上で処理済み
  grep -q "^\[workspace\]" "${dir}Cargo.toml" 2>/dev/null && continue

  [ -f "${dir}src/lib.rs" ] || continue
  [ -d "solutions/${name}" ] || { echo "== $name: solutions/ がありません（スキップ）=="; continue; }

  echo "== $name =="
  # 解答ディレクトリにある全ての .rs を、対応する演習ファイルへ重ねる
  while IFS= read -r sol; do
    rel="${sol#solutions/${name}/}"
    [ -f "${dir}${rel}" ] && overlay "${dir}${rel}" "$sol"
  done < <(find "solutions/${name}" -name '*.rs')

  ok=1
  cargo test --quiet -p "$name" || ok=0
  if [ "$RUN_FMT" -eq 1 ]; then
    cargo fmt -p "$name" --check || ok=0
  fi
  if [ "$RUN_CLIPPY" -eq 1 ]; then
    cargo clippy -p "$name" -- -D warnings || ok=0
  fi

  if [ "$ok" -eq 1 ]; then
    echo "  OK"
    PASSED=$((PASSED + 1))
  else
    echo "  NG"
    FAILED=$((FAILED + 1))
    FAILED_NAMES+=("$name")
  fi
done

echo
echo "Passed: $PASSED  Failed: $FAILED"
if [ "$FAILED" -gt 0 ]; then
  echo "Failed exercises: ${FAILED_NAMES[*]}"
  exit 1
fi
