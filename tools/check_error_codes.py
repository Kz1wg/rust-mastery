#!/usr/bin/env python3
"""book/src 内の ```rust,compile_fail,EXXXX ブロックが、実際にそのエラーコードで
コンパイルに失敗することを検証する。

背景: stable の rustdoc（mdbook test）は compile_fail のエラーコード一致を検証しない
（ARCHITECTURE.md §3.3）。教材の解説と実際の rustc 出力のずれを防ぐための補助ツール。

使い方（どのディレクトリから実行してもよい）:
    python3 tools/check_error_codes.py [検索するディレクトリ]
省略時は、このスクリプトの位置を基準に <リポジトリ>/book/src を検索する。
依存: Python 3 標準ライブラリ、rustc
"""

import re
import subprocess
import sys
import tempfile
from pathlib import Path

FENCE = re.compile(r"^```(?P<info>[^\n]*)\n(?P<body>.*?)^```[ \t]*$", re.S | re.M)
CODE = re.compile(r"\bE\d{4}\b")


def wrap(body: str) -> str:
    # rustdoc と同様、fn main が無ければ全体を main で包む
    if "fn main" in body:
        return "#![allow(unused)]\n" + body
    return "#![allow(unused)]\nfn main() {\n" + body + "\n}\n"


def main() -> int:
    default_root = Path(__file__).resolve().parent.parent / "book" / "src"
    root = Path(sys.argv[1]) if len(sys.argv) > 1 else default_root
    if not root.is_dir():
        print(f"エラー: ディレクトリが見つかりません: {root}", file=sys.stderr)
        return 2
    checked = failed = 0
    for md in sorted(root.rglob("*.md")):
        text = md.read_text(encoding="utf-8")
        for m in FENCE.finditer(text):
            info = m.group("info")
            if "compile_fail" not in info:
                continue
            codes = CODE.findall(info)
            line = text[: m.start()].count("\n") + 1
            with tempfile.TemporaryDirectory() as d:
                src = Path(d) / "snippet.rs"
                src.write_text(wrap(m.group("body")), encoding="utf-8")
                r = subprocess.run(
                    ["rustc", "--edition", "2021", "--emit=metadata",
                     "--out-dir", d, str(src)],
                    capture_output=True, text=True,
                )
            checked += 1
            if r.returncode == 0:
                print(f"NG  {md}:{line}: コンパイルが通ってしまった")
                failed += 1
                continue
            if not codes:
                print(f"--  {md}:{line}: エラーコード未指定（コンパイル失敗のみ確認）")
                continue
            missing = [c for c in codes if f"error[{c}]" not in r.stderr]
            if missing:
                found = sorted(set(re.findall(r"error\[(E\d{4})\]", r.stderr)))
                print(f"NG  {md}:{line}: 期待 {codes} / 実際 {found}")
                failed += 1
            else:
                print(f"OK  {md}:{line}: {codes}")
    print(f"\n{checked} blocks checked, {failed} mismatched  (root: {root})")
    if checked == 0:
        # 1つも検証していないのに成功扱いにしない
        print("エラー: compile_fail ブロックが1つも見つかりませんでした", file=sys.stderr)
        return 2
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
