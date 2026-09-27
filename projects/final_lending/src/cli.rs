//! コマンドラインの解釈と、ファイルの読み書き。
//!
//! 役割の分担:
//! - `parse_args`: 文字列の並び → `Invocation`（ファイルにも時計にも触らない）
//! - `execute`: `Command` を `Library` に対して実行し、表示する文字列を返す（ファイルに触らない）
//! - `run`: ファイルを読む → execute → 必要なら書く。I/O はここに集める
//!
//! main.rs は `run` を呼んで、結果を表示し終了コードを決めるだけ（Lesson 12-2）。

use std::fmt;
use std::fmt::Write as _;
use std::io;
use std::path::{Path, PathBuf};

use crate::date::Date;
use crate::ids::{BookId, MemberId};
use crate::library::{BookState, LendingError, Library};
use crate::member::MemberKind;
use crate::store::{self, StoreError};

pub const USAGE: &str = "\
usage: lending [--data FILE] [--today YYYY-MM-DD] <command>

commands:
  add-book   <book-id> <title>                 本を登録する
  add-member <member-id> <general|staff> <name> 利用者を登録する
  borrow     <member-id> <book-id>             本を貸し出す
  return     <book-id>                         本を返却する
  loans      <member-id>                       利用者が借りている本
  overdue                                      延滞している本
  list                                         全ての本と状態

options:
  --data FILE           データファイル（既定: lending.tsv）
  --today YYYY-MM-DD    今日の日付として使う日（既定: システムの日付。UTC）";

/// 実行するコマンド。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    AddBook {
        id: BookId,
        title: String,
    },
    AddMember {
        id: MemberId,
        kind: MemberKind,
        name: String,
    },
    Borrow {
        member: MemberId,
        book: BookId,
    },
    Return {
        book: BookId,
    },
    Loans {
        member: MemberId,
    },
    Overdue,
    List,
}

impl Command {
    /// データを変えるコマンドか（変えるものだけ、実行後にファイルへ書き戻す）。
    pub fn changes_data(&self) -> bool {
        matches!(
            self,
            Command::AddBook { .. }
                | Command::AddMember { .. }
                | Command::Borrow { .. }
                | Command::Return { .. }
        )
    }
}

/// 引数を解釈した結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Invocation {
    pub data: PathBuf,
    /// `--today` で指定された日付。無ければ呼ぶ側がシステムの日付を使う。
    pub today: Option<Date>,
    pub command: Command,
}

/// アプリケーション全体の失敗。
///
/// ライブラリ部分の失敗（LendingError / StoreError）を包み、I/O の失敗には
/// どのファイルだったかを添える（Lesson 06-4: application 側のエラーは「利用者に伝える」ためのもの）。
#[derive(Debug)]
pub enum CliError {
    /// 引数の誤り。使い方を表示して終わる。
    Usage(String),
    Io {
        path: PathBuf,
        source: io::Error,
    },
    Store {
        path: PathBuf,
        source: StoreError,
    },
    Lending(LendingError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::Usage(msg) => write!(f, "{msg}"),
            CliError::Io { path, source } => write!(f, "{}: {source}", path.display()),
            CliError::Store { path, source } => write!(f, "{}: {source}", path.display()),
            CliError::Lending(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for CliError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CliError::Usage(_) => None,
            CliError::Io { source, .. } => Some(source),
            CliError::Store { source, .. } => Some(source),
            CliError::Lending(e) => Some(e),
        }
    }
}

impl From<LendingError> for CliError {
    fn from(e: LendingError) -> Self {
        CliError::Lending(e)
    }
}

fn usage(msg: impl Into<String>) -> CliError {
    CliError::Usage(msg.into())
}

/// `FromStr` の型として読む。失敗したら、その型のエラーメッセージで Usage にする。
fn arg<T>(s: &str) -> Result<T, CliError>
where
    T: std::str::FromStr,
    T::Err: fmt::Display,
{
    s.parse().map_err(|e: T::Err| usage(e.to_string()))
}

/// 引数（プログラム名を除く）を解釈する。
pub fn parse_args(args: &[String]) -> Result<Invocation, CliError> {
    let mut data = PathBuf::from("lending.tsv");
    let mut today = None;
    let mut words: Vec<&str> = Vec::new();

    let mut iter = args.iter();
    while let Some(a) = iter.next() {
        match a.as_str() {
            "--data" => {
                let v = iter
                    .next()
                    .ok_or_else(|| usage("--data の値がありません"))?;
                data = PathBuf::from(v);
            }
            "--today" => {
                let v = iter
                    .next()
                    .ok_or_else(|| usage("--today の値がありません"))?;
                today = Some(arg::<Date>(v)?);
            }
            s if s.starts_with("--") => return Err(usage(format!("知らないオプションです: {s}"))),
            s => words.push(s),
        }
    }

    let command = match words[..] {
        ["add-book", id, title] => Command::AddBook {
            id: arg(id)?,
            title: title.to_string(),
        },
        ["add-member", id, kind, name] => Command::AddMember {
            id: arg(id)?,
            kind: arg(kind)?,
            name: name.to_string(),
        },
        ["borrow", member, book] => Command::Borrow {
            member: arg(member)?,
            book: arg(book)?,
        },
        ["return", book] => Command::Return { book: arg(book)? },
        ["loans", member] => Command::Loans {
            member: arg(member)?,
        },
        ["overdue"] => Command::Overdue,
        ["list"] => Command::List,
        [] => return Err(usage("コマンドを指定してください")),
        [cmd, ..] => {
            return Err(usage(format!(
                "コマンド {cmd:?} の名前か、引数の数が正しくありません"
            )))
        }
    };

    Ok(Invocation {
        data,
        today,
        command,
    })
}

/// コマンドを実行し、表示する文字列を返す。
pub fn execute(
    library: &mut Library,
    command: &Command,
    today: Date,
) -> Result<String, LendingError> {
    let mut out = String::new();
    // String への write! は失敗しないので、結果は捨ててよい
    match command {
        Command::AddBook { id, title } => {
            library.add_book(id.clone(), title)?;
            let _ = write!(out, "本 {id}「{title}」を登録しました");
        }
        Command::AddMember { id, kind, name } => {
            library.add_member(id.clone(), name, *kind)?;
            let _ = write!(out, "利用者 {id}（{name}, {kind}）を登録しました");
        }
        Command::Borrow { member, book } => {
            let due = library.borrow(member, book, today)?;
            let _ = write!(
                out,
                "本 {book} を {member} に貸し出しました。返却期限は {due} です"
            );
        }
        Command::Return { book } => {
            let receipt = library.return_book(book, today)?;
            let _ = write!(
                out,
                "本 {book} の返却を受け付けました（{}）",
                receipt.member
            );
            if receipt.overdue_days > 0 {
                let _ = write!(out, "。{}日遅れです", receipt.overdue_days);
            }
        }
        Command::Loans { member } => {
            if library.member(member).is_none() {
                return Err(LendingError::UnknownMember(member.clone()));
            }
            for (book, loan) in library.loans_of(member) {
                let _ = writeln!(out, "{}\t{}\t期限 {}", book.id, book.title, loan.due);
            }
            if out.is_empty() {
                out.push_str("借りている本はありません");
            }
        }
        Command::Overdue => {
            for (book, loan) in library.overdue(today) {
                let _ = writeln!(
                    out,
                    "{}\t{}\t{}\t{}日",
                    book.id,
                    book.title,
                    loan.member,
                    loan.overdue_days(today)
                );
            }
            if out.is_empty() {
                out.push_str("延滞している本はありません");
            }
        }
        Command::List => {
            for book in library.books() {
                let state = match &book.state {
                    BookState::Available => "貸出可".to_string(),
                    BookState::OnLoan(loan) => {
                        format!("貸出中（{}、期限 {}）", loan.member, loan.due)
                    }
                };
                let _ = writeln!(out, "{}\t{}\t{state}", book.id, book.title);
            }
            if out.is_empty() {
                out.push_str("本は登録されていません");
            }
        }
    }
    Ok(out.trim_end().to_string())
}

/// データファイルを読む。ファイルがまだ無ければ、空の図書室から始める。
fn load_file(path: &Path) -> Result<Library, CliError> {
    match std::fs::read_to_string(path) {
        Ok(text) => store::load(&text).map_err(|source| CliError::Store {
            path: path.to_path_buf(),
            source,
        }),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Library::new()),
        Err(source) => Err(CliError::Io {
            path: path.to_path_buf(),
            source,
        }),
    }
}

/// データファイルに書く。
///
/// 一時ファイルに書いてから名前を付け替える。書いている途中でプログラムが止まっても、
/// 元のファイルが半分だけ書かれた状態で残らないようにするため。
fn save_file(path: &Path, library: &Library) -> Result<(), CliError> {
    let io_err = |source| CliError::Io {
        path: path.to_path_buf(),
        source,
    };
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    std::fs::write(&tmp, store::save(library)).map_err(io_err)?;
    std::fs::rename(&tmp, path).map_err(io_err)
}

/// ファイルを読み、コマンドを実行し、データが変わったなら書き戻す。
pub fn run(invocation: &Invocation, today: Date) -> Result<String, CliError> {
    let mut library = load_file(&invocation.data)?;
    let output = execute(&mut library, &invocation.command, today)?;
    if invocation.command.changes_data() {
        save_file(&invocation.data, &library)?;
    }
    Ok(output)
}
