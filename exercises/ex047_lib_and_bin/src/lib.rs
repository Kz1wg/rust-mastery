//! Lesson 12-2: lib / bin の分離
//!
//! ロジックは lib.rs に置き、main.rs は「引数を読む・run を呼ぶ・終了コードを決める」だけ。
//! テストは lib.rs の公開APIだけを呼ぶ（main.rs の中身は統合テストから呼べない）。

/// 空白区切りの単語数を数える。I/O を含まない純粋な関数なのでテストが簡単。
pub fn count_words(content: &str) -> usize {
    todo!("空白で区切って単語数を数えてください")
}

/// ファイルを読み、"N words" という文字列を返す。
/// 失敗は panic させず Result で返す（呼び出し側が扱いを決められる）。
pub fn run(path: &str) -> Result<String, std::io::Error> {
    todo!("ファイルを読み、count_words の結果を \"N words\" の形式で返してください")
}
