//! P1: wordstat — テキストの単語を数え、多い順に表示する CLI ツール。
//!
//! ロジックは全部ここ（lib.rs）にあり、main.rs は引数を渡して結果を表示するだけ。
//! 入力は `impl Read` で受け取るので、テストではファイルを用意せずに文字列を渡せる（Lesson 13-1）。

use std::collections::HashMap;
use std::fmt;
use std::io::Read;

/// コマンドライン引数を解析した結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Args {
    /// 読み込むファイルのパス。
    pub path: String,
    /// 上位何件を表示するか（既定は 10）。
    pub top: usize,
    /// 大文字・小文字を区別しないか（既定は false）。
    pub ignore_case: bool,
}

/// wordstat の失敗。
#[derive(Debug)]
pub enum CliError {
    /// ファイルのパスが指定されていない。
    MissingPath,
    /// パスが2つ以上指定された（2つ目の値を持つ）。
    UnexpectedArgument(String),
    /// 知らないオプション（例: `--foo`）。
    UnknownOption(String),
    /// `--top` の値が無い、または数として読めない（読めなかった値を持つ。値が無ければ空文字列）。
    InvalidTop(String),
    /// 入力の読み込みに失敗した。
    Io(std::io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::MissingPath => write!(f, "ファイルが指定されていません"),
            CliError::UnexpectedArgument(a) => write!(f, "余計な引数があります: {a}"),
            CliError::UnknownOption(o) => write!(f, "知らないオプションです: {o}"),
            CliError::InvalidTop(v) => write!(f, "--top には正の整数を指定してください（{v:?}）"),
            CliError::Io(e) => write!(f, "読み込みに失敗しました: {e}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(e: std::io::Error) -> Self {
        CliError::Io(e)
    }
}

/// 1つの単語と、その出現回数。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WordCount {
    pub word: String,
    pub count: usize,
}

/// 引数（プログラム名を除いたもの）を解析する。
///
/// - `--top N`: 上位 N 件（N は 1 以上の整数）
/// - `--ignore-case`: 大文字・小文字を区別しない
/// - それ以外の `--` で始まるものは UnknownOption
/// - `--` で始まらないものはファイルのパス（ちょうど1つ）
pub fn parse_args(args: &[String]) -> Result<Args, CliError> {
    let mut path: Option<String> = None;
    let mut top = 10;
    let mut ignore_case = false;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        let _ = (&mut path, &mut top, &mut ignore_case, &mut iter, arg);
        todo!("arg の種類ごとに、上のルールどおり path / top / ignore_case を設定するか、Err を返してください")
    }

    let path = path.ok_or(CliError::MissingPath)?;
    Ok(Args {
        path,
        top,
        ignore_case,
    })
}

/// 単語を正規化する: 前後の記号（英数字以外）を取り除き、必要なら小文字にする。
/// 何も残らなければ None。
pub fn normalize(word: &str, ignore_case: bool) -> Option<String> {
    todo!("前後の英数字以外の文字を取り除き、空なら None、ignore_case なら小文字にして Some で返してください")
}

/// テキストの単語を数え、出現回数の多い順（同じ回数なら単語の辞書順）に並べて返す。
pub fn count_words(text: &str, ignore_case: bool) -> Vec<WordCount> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for raw in text.split_whitespace() {
        if let Some(word) = normalize(raw, ignore_case) {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    let mut result: Vec<WordCount> = counts
        .into_iter()
        .map(|(word, count)| WordCount { word, count })
        .collect();
    let _ = &mut result;
    todo!("回数の多い順、同じ回数なら word の辞書順に並べて返してください")
}

/// 入力を読み、上位 args.top 件を「単語<TAB>回数」の行にして返す（各行は改行で終わる）。
pub fn run<R: Read>(args: &Args, mut input: R) -> Result<String, CliError> {
    let mut text = String::new();
    input.read_to_string(&mut text)?;
    let _ = &text;
    todo!("count_words の結果の上位 args.top 件を、単語<TAB>回数 の行にまとめて返してください")
}
