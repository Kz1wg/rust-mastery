use std::collections::HashMap;
use std::fmt;
use std::fmt::Write as _;
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
    /// `--top` の値が無い、または数として読めない。
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

pub fn parse_args(args: &[String]) -> Result<Args, CliError> {
    let mut path: Option<String> = None;
    let mut top = 10;
    let mut ignore_case = false;

    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--ignore-case" => ignore_case = true,
            "--top" => {
                let value = iter.next().cloned().unwrap_or_default();
                top = match value.parse::<usize>() {
                    Ok(n) if n > 0 => n,
                    _ => return Err(CliError::InvalidTop(value)),
                };
            }
            other if other.starts_with("--") => {
                return Err(CliError::UnknownOption(other.to_string()));
            }
            other => {
                if path.is_some() {
                    return Err(CliError::UnexpectedArgument(other.to_string()));
                }
                path = Some(other.to_string());
            }
        }
    }

    let path = path.ok_or(CliError::MissingPath)?;
    Ok(Args {
        path,
        top,
        ignore_case,
    })
}

pub fn normalize(word: &str, ignore_case: bool) -> Option<String> {
    let trimmed = word.trim_matches(|c: char| !c.is_alphanumeric());
    if trimmed.is_empty() {
        return None;
    }
    Some(if ignore_case {
        trimmed.to_lowercase()
    } else {
        trimmed.to_string()
    })
}

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
    result.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| a.word.cmp(&b.word)));
    result
}

pub fn run<R: Read>(args: &Args, mut input: R) -> Result<String, CliError> {
    let mut text = String::new();
    input.read_to_string(&mut text)?;
    // format! で1行ずつ String を作って collect すると、行ごとに確保が発生する。
    // 1つの String に writeln! で直接書き足す（clippy の format_collect が教えてくれる）。
    let mut output = String::new();
    for wc in count_words(&text, args.ignore_case)
        .into_iter()
        .take(args.top)
    {
        writeln!(output, "{}\t{}", wc.word, wc.count).expect("String への書き込みは失敗しない");
    }
    Ok(output)
}
