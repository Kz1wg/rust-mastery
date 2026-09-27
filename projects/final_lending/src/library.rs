//! 図書室（本と利用者の集まり）と、貸出・返却のルール。
//!
//! ルールを確かめるのはこのモジュールだけ。CLI や保存形式のことは知らない。
//! 「今日の日付」は引数で受け取る（時計を読まない）ので、テストでは好きな日付を渡せる。

use std::collections::BTreeMap;
use std::fmt;

use crate::date::Date;
use crate::ids::{BookId, MemberId};
use crate::member::{Member, MemberKind};

/// 貸出中の記録。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loan {
    pub member: MemberId,
    pub since: Date,
    pub due: Date,
}

impl Loan {
    /// `today` の時点で何日延滞しているか（延滞していなければ 0）。
    pub fn overdue_days(&self, today: Date) -> u32 {
        today.days_since(self.due).max(0) as u32
    }
}

/// 本の状態。
///
/// 「貸出中なのに借りた人がいない」「返却済みなのに返却期限がある」といった状態を
/// 作れないよう、借りた人と期限は `OnLoan` の中にだけ持たせている（Lesson 02-1, 03-1）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BookState {
    Available,
    OnLoan(Loan),
}

/// 本（1冊）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub state: BookState,
}

/// 返却の結果。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnReceipt {
    pub book: BookId,
    pub member: MemberId,
    /// 何日遅れて返されたか（期限内なら 0）。
    pub overdue_days: u32,
}

/// 貸出・返却などの操作が、ルールによって断られた理由。
///
/// どれも「利用者に伝えて、やり直してもらう」種類の失敗なので、1つの enum にまとめている。
/// ファイルが読めない、などの失敗はここには入れない（それは store / CLI の失敗）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LendingError {
    UnknownBook(BookId),
    UnknownMember(MemberId),
    DuplicateBook(BookId),
    DuplicateMember(MemberId),
    /// その本はすでに貸出中。
    AlreadyOnLoan(BookId),
    /// その本は貸出中ではない（返却しようとした）。
    NotOnLoan(BookId),
    /// 借りられる冊数の上限に達している。
    LoanLimitReached {
        member: MemberId,
        limit: usize,
    },
    /// 延滞中の本があるので、新しく借りられない。
    HasOverdue {
        member: MemberId,
        book: BookId,
    },
    /// 返却日が貸出日より前。
    ReturnBeforeLoan {
        book: BookId,
        since: Date,
    },
}

impl fmt::Display for LendingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use LendingError::*;
        match self {
            UnknownBook(b) => write!(f, "本 {b} は登録されていません"),
            UnknownMember(m) => write!(f, "利用者 {m} は登録されていません"),
            DuplicateBook(b) => write!(f, "本 {b} はすでに登録されています"),
            DuplicateMember(m) => write!(f, "利用者 {m} はすでに登録されています"),
            AlreadyOnLoan(b) => write!(f, "本 {b} は貸出中です"),
            NotOnLoan(b) => write!(f, "本 {b} は貸出中ではありません"),
            LoanLimitReached { member, limit } => {
                write!(f, "利用者 {member} は上限の {limit} 冊を借りています")
            }
            HasOverdue { member, book } => {
                write!(
                    f,
                    "利用者 {member} は本 {book} を延滞しているため、借りられません"
                )
            }
            ReturnBeforeLoan { book, since } => {
                write!(f, "本 {book} の返却日が貸出日（{since}）より前です")
            }
        }
    }
}

impl std::error::Error for LendingError {}

/// 図書室。本と利用者を持ち、貸出・返却のルールを守らせる。
///
/// 「誰が何冊借りているか」は、本の状態から数える。利用者の側にも貸出の一覧を持たせると、
/// 同じ事実を2か所で管理することになり、ずれる可能性が生まれるため（本文 18-2）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Library {
    // BTreeMap にしているのは、一覧や保存のときに ID 順で並ぶようにするため。
    books: BTreeMap<BookId, Book>,
    members: BTreeMap<MemberId, Member>,
}

impl Library {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_book(&mut self, id: BookId, title: &str) -> Result<(), LendingError> {
        if self.books.contains_key(&id) {
            return Err(LendingError::DuplicateBook(id));
        }
        let book = Book {
            id: id.clone(),
            title: title.to_string(),
            state: BookState::Available,
        };
        self.books.insert(id, book);
        Ok(())
    }

    pub fn add_member(
        &mut self,
        id: MemberId,
        name: &str,
        kind: MemberKind,
    ) -> Result<(), LendingError> {
        if self.members.contains_key(&id) {
            return Err(LendingError::DuplicateMember(id));
        }
        let member = Member {
            id: id.clone(),
            name: name.to_string(),
            kind,
        };
        self.members.insert(id, member);
        Ok(())
    }

    pub fn book(&self, id: &BookId) -> Option<&Book> {
        self.books.get(id)
    }

    pub fn member(&self, id: &MemberId) -> Option<&Member> {
        self.members.get(id)
    }

    /// 全ての本（ID 順）。
    pub fn books(&self) -> impl Iterator<Item = &Book> {
        self.books.values()
    }

    /// 全ての利用者（ID 順）。
    pub fn members(&self) -> impl Iterator<Item = &Member> {
        self.members.values()
    }

    /// その利用者が借りている本と、貸出の記録。
    pub fn loans_of<'a>(
        &'a self,
        member: &'a MemberId,
    ) -> impl Iterator<Item = (&'a Book, &'a Loan)> + 'a {
        self.books
            .values()
            .filter_map(move |book| match &book.state {
                BookState::OnLoan(loan) if &loan.member == member => Some((book, loan)),
                _ => None,
            })
    }

    /// `today` の時点で延滞している貸出（本の ID 順）。
    pub fn overdue(&self, today: Date) -> impl Iterator<Item = (&Book, &Loan)> {
        self.books
            .values()
            .filter_map(move |book| match &book.state {
                BookState::OnLoan(loan) if loan.overdue_days(today) > 0 => Some((book, loan)),
                _ => None,
            })
    }

    /// 本を貸し出す。成功したら返却期限を返す。
    ///
    /// 確かめる順番: 利用者がいる → 本がある → 本が貸出可能 → 延滞がない → 上限に達していない。
    /// どれか1つでも満たさなければ、何も変えずに Err を返す。
    pub fn borrow(
        &mut self,
        member_id: &MemberId,
        book_id: &BookId,
        today: Date,
    ) -> Result<Date, LendingError> {
        let member = self
            .members
            .get(member_id)
            .ok_or_else(|| LendingError::UnknownMember(member_id.clone()))?;
        let policy = member.kind.policy();

        let book = self
            .books
            .get(book_id)
            .ok_or_else(|| LendingError::UnknownBook(book_id.clone()))?;
        if matches!(book.state, BookState::OnLoan(_)) {
            return Err(LendingError::AlreadyOnLoan(book_id.clone()));
        }

        let mut count = 0;
        for (book, loan) in self.loans_of(member_id) {
            if loan.overdue_days(today) > 0 {
                return Err(LendingError::HasOverdue {
                    member: member_id.clone(),
                    book: book.id.clone(),
                });
            }
            count += 1;
        }
        if count >= policy.max_loans {
            return Err(LendingError::LoanLimitReached {
                member: member_id.clone(),
                limit: policy.max_loans,
            });
        }

        // ここまで来たら、ルールはすべて満たしている。ここで初めて状態を変える。
        let due = today.add_days(policy.loan_days);
        let book = self.books.get_mut(book_id).expect("上で存在を確かめている");
        book.state = BookState::OnLoan(Loan {
            member: member_id.clone(),
            since: today,
            due,
        });
        Ok(due)
    }

    /// 本を返却する。
    pub fn return_book(
        &mut self,
        book_id: &BookId,
        today: Date,
    ) -> Result<ReturnReceipt, LendingError> {
        let book = self
            .books
            .get_mut(book_id)
            .ok_or_else(|| LendingError::UnknownBook(book_id.clone()))?;
        let BookState::OnLoan(loan) = &book.state else {
            return Err(LendingError::NotOnLoan(book_id.clone()));
        };
        if today < loan.since {
            return Err(LendingError::ReturnBeforeLoan {
                book: book_id.clone(),
                since: loan.since,
            });
        }

        let receipt = ReturnReceipt {
            book: book_id.clone(),
            member: loan.member.clone(),
            overdue_days: loan.overdue_days(today),
        };
        book.state = BookState::Available;
        Ok(receipt)
    }

    /// 保存したデータから、本を状態ごと戻す（store モジュール専用）。
    ///
    /// ここでは貸出のルール（上限・延滞）を**確かめない**。保存された時点ではルールを満たしていたし、
    /// その後にルールが変わっていても（上限を減らした、など）、過去の貸出が読めなくなっては困るため。
    /// 確かめるのは、データとして矛盾していないこと（ID の重複、存在しない利用者への貸出）だけ。
    pub(crate) fn restore_book(&mut self, book: Book) -> Result<(), LendingError> {
        if self.books.contains_key(&book.id) {
            return Err(LendingError::DuplicateBook(book.id));
        }
        if let BookState::OnLoan(loan) = &book.state {
            if !self.members.contains_key(&loan.member) {
                return Err(LendingError::UnknownMember(loan.member.clone()));
            }
        }
        self.books.insert(book.id.clone(), book);
        Ok(())
    }
}
