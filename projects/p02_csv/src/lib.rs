//! P2: 小さな CSV パーサ。
//!
//! 対応する形式（範囲を絞っている）:
//! - カンマ区切り。1行 = 1レコード（引用符の中の改行は扱わない）
//! - `"` で囲んだフィールドの中では、カンマを文字として使える
//! - 囲んだフィールドの中の `""` は、`"` 1文字を表す
//! - 空行は読み飛ばす
//!
//! 1行の解析は、Chapter 03 の状態機械として書く（State enum）。

use std::fmt;
use std::io::BufRead;
use std::rc::Rc;

/// 解析の失敗。どの行・どの列で起きたかを持つ（行・列は 1 から数える）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CsvError {
    /// 引用符で始まったフィールドが、行の終わりまでに閉じられていない。
    UnterminatedQuote { line: usize },
    /// 引用符で囲まれていないフィールドの途中に `"` がある（例: `ab"c`）。
    UnexpectedQuote { line: usize, column: usize },
    /// 閉じ引用符の直後に、カンマ以外の文字がある（例: `"ab"c`）。
    CharAfterClosingQuote { line: usize, column: usize },
    /// フィールドの数がヘッダーと違う。
    FieldCountMismatch {
        line: usize,
        expected: usize,
        found: usize,
    },
    /// ヘッダー行が無い（入力が空）。
    MissingHeader,
    /// 読み込みに失敗した。
    Io(std::io::ErrorKind),
}

impl fmt::Display for CsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CsvError::UnterminatedQuote { line } => {
                write!(f, "{line}行目: 引用符が閉じられていません")
            }
            CsvError::UnexpectedQuote { line, column } => {
                write!(
                    f,
                    "{line}行目{column}列目: フィールドの途中に引用符があります"
                )
            }
            CsvError::CharAfterClosingQuote { line, column } => {
                write!(f, "{line}行目{column}列目: 閉じ引用符の後に文字があります")
            }
            CsvError::FieldCountMismatch {
                line,
                expected,
                found,
            } => {
                write!(
                    f,
                    "{line}行目: フィールドが{found}個あります（ヘッダーは{expected}個）"
                )
            }
            CsvError::MissingHeader => write!(f, "ヘッダー行がありません"),
            CsvError::Io(kind) => write!(f, "読み込みに失敗しました: {kind:?}"),
        }
    }
}

impl std::error::Error for CsvError {}

/// 1行を解析している途中の「今どこにいるか」。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    /// フィールドの始まり（まだ1文字も読んでいない）
    FieldStart,
    /// 引用符で囲まれていないフィールドの途中
    Unquoted,
    /// 引用符で囲まれたフィールドの途中
    Quoted,
    /// 囲まれたフィールドの中で `"` を読んだ直後（閉じ引用符か、`""` の1文字目）
    QuoteInQuoted,
}

/// 1行を解析して、フィールドの一覧を返す。line_no はエラーに含める行番号。
///
/// 状態ごとの動き:
///
/// | 今の状態 | 読んだ文字 | すること | 次の状態 |
/// | --- | --- | --- | --- |
/// | FieldStart | `"` | 何もしない | Quoted |
/// | FieldStart | `,` | 空のフィールドを確定 | FieldStart |
/// | FieldStart | その他 | 文字を足す | Unquoted |
/// | Unquoted | `,` | フィールドを確定 | FieldStart |
/// | Unquoted | `"` | エラー（UnexpectedQuote） | — |
/// | Unquoted | その他 | 文字を足す | Unquoted |
/// | Quoted | `"` | 何もしない | QuoteInQuoted |
/// | Quoted | その他（`,` も含む） | 文字を足す | Quoted |
/// | QuoteInQuoted | `"` | `"` を1文字足す | Quoted |
/// | QuoteInQuoted | `,` | フィールドを確定 | FieldStart |
/// | QuoteInQuoted | その他 | エラー（CharAfterClosingQuote） | — |
///
/// 行の終わりでは、Quoted ならエラー（UnterminatedQuote）、それ以外は最後のフィールドを確定する。
pub fn parse_line(line: &str, line_no: usize) -> Result<Vec<String>, CsvError> {
    let mut fields: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut state = State::FieldStart;

    for (index, c) in line.chars().enumerate() {
        let column = index + 1;
        let _ = (&mut fields, &mut current, &mut state, column, c);
        todo!("上の表のとおり、state と c の組み合わせごとに処理を書いてください（match を使う）")
    }

    todo!("行の終わりの処理: Quoted なら UnterminatedQuote、そうでなければ最後のフィールドを足して Ok を返してください")
}

/// ヘッダー名で値を取り出せる1レコード。
/// ヘッダーは全レコードで同じなので、Rc で共有してコピーを避けている（Lesson 10-3）。
#[derive(Debug, Clone)]
pub struct Record {
    headers: Rc<Vec<String>>,
    values: Vec<String>,
}

impl Record {
    /// ヘッダー名に対応する値を返す。無い名前なら None。
    pub fn get(&self, name: &str) -> Option<&str> {
        todo!("headers の中で name の位置を探し、同じ位置の values を返してください")
    }

    /// 値の一覧（ヘッダーと同じ順）。
    pub fn values(&self) -> &[String] {
        &self.values
    }
}

/// CSV を1レコードずつ読むイテレータ。1行目をヘッダーとして読む。
pub struct CsvReader<R> {
    lines: std::io::Lines<R>,
    headers: Rc<Vec<String>>,
    line_no: usize,
}

impl<R: BufRead> CsvReader<R> {
    /// 1行目を読んでヘッダーにする。空行は読み飛ばす。ヘッダーが無ければ MissingHeader。
    pub fn new(reader: R) -> Result<Self, CsvError> {
        let mut lines = reader.lines();
        let mut line_no = 0;
        loop {
            line_no += 1;
            match lines.next() {
                None => return Err(CsvError::MissingHeader),
                Some(Err(e)) => return Err(CsvError::Io(e.kind())),
                Some(Ok(line)) if line.trim().is_empty() => continue,
                Some(Ok(line)) => {
                    let headers = parse_line(&line, line_no)?;
                    return Ok(CsvReader {
                        lines,
                        headers: Rc::new(headers),
                        line_no,
                    });
                }
            }
        }
    }

    /// ヘッダーの一覧。
    pub fn headers(&self) -> &[String] {
        &self.headers
    }
}

impl<R: BufRead> Iterator for CsvReader<R> {
    type Item = Result<Record, CsvError>;

    /// 次のレコードを返す。空行は読み飛ばす。
    /// フィールドの数がヘッダーと違えば FieldCountMismatch。
    fn next(&mut self) -> Option<Self::Item> {
        todo!("次の空でない行を読み、parse_line して、フィールド数を確かめてから Record を返してください（行番号の数え方に注意）")
    }
}
