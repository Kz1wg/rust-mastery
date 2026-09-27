//! 要件（本文 18-1 の R1〜R10）ごとの受け入れテスト。
//! テスト名の先頭の r1_ などが、要件の番号に対応している。

use final_lending::{BookId, BookState, Date, LendingError, Library, MemberId, MemberKind};

fn d(s: &str) -> Date {
    s.parse().unwrap()
}
fn book(s: &str) -> BookId {
    BookId::new(s).unwrap()
}
fn member(s: &str) -> MemberId {
    MemberId::new(s).unwrap()
}

/// 本 B1〜B15 と、一般の利用者 M1・職員 S1 がいる図書室。
fn library() -> Library {
    let mut lib = Library::new();
    for i in 1..=15 {
        lib.add_book(book(&format!("B{i}")), &format!("本その{i}"))
            .unwrap();
    }
    lib.add_member(member("M1"), "山田", MemberKind::General)
        .unwrap();
    lib.add_member(member("S1"), "佐藤", MemberKind::Staff)
        .unwrap();
    lib
}

#[test]
fn r1_register_books_and_reject_duplicates() {
    let mut lib = Library::new();
    lib.add_book(book("B1"), "Rustの本").unwrap();
    assert_eq!(lib.book(&book("B1")).unwrap().title, "Rustの本");
    assert_eq!(
        lib.add_book(book("B1"), "別の本"),
        Err(LendingError::DuplicateBook(book("B1")))
    );
    // 断られたとき、元の登録は変わらない
    assert_eq!(lib.book(&book("B1")).unwrap().title, "Rustの本");
}

#[test]
fn r2_register_members_and_reject_duplicates() {
    let mut lib = library();
    assert_eq!(lib.member(&member("S1")).unwrap().kind, MemberKind::Staff);
    assert_eq!(
        lib.add_member(member("M1"), "別人", MemberKind::Staff),
        Err(LendingError::DuplicateMember(member("M1")))
    );
}

#[test]
fn r3_due_date_depends_on_member_kind() {
    let mut lib = library();
    assert_eq!(
        lib.borrow(&member("M1"), &book("B1"), d("2026-09-25")),
        Ok(d("2026-10-09")) // 一般: 14日
    );
    assert_eq!(
        lib.borrow(&member("S1"), &book("B2"), d("2026-09-25")),
        Ok(d("2026-10-25")) // 職員: 30日
    );
    assert!(matches!(
        lib.book(&book("B1")).unwrap().state,
        BookState::OnLoan(_)
    ));
}

#[test]
fn r4_book_on_loan_cannot_be_borrowed_again() {
    let mut lib = library();
    lib.borrow(&member("M1"), &book("B1"), d("2026-09-25"))
        .unwrap();
    assert_eq!(
        lib.borrow(&member("S1"), &book("B1"), d("2026-09-26")),
        Err(LendingError::AlreadyOnLoan(book("B1")))
    );
}

#[test]
fn r5_loan_limit_depends_on_member_kind() {
    let mut lib = library();
    let today = d("2026-09-25");
    for i in 1..=3 {
        lib.borrow(&member("M1"), &book(&format!("B{i}")), today)
            .unwrap();
    }
    assert_eq!(
        lib.borrow(&member("M1"), &book("B4"), today),
        Err(LendingError::LoanLimitReached {
            member: member("M1"),
            limit: 3
        })
    );
    // 職員は 10 冊まで
    for i in 4..=13 {
        lib.borrow(&member("S1"), &book(&format!("B{i}")), today)
            .unwrap();
    }
    assert_eq!(
        lib.borrow(&member("S1"), &book("B14"), today),
        Err(LendingError::LoanLimitReached {
            member: member("S1"),
            limit: 10
        })
    );
}

#[test]
fn r5_returning_frees_a_slot() {
    let mut lib = library();
    let today = d("2026-09-25");
    for i in 1..=3 {
        lib.borrow(&member("M1"), &book(&format!("B{i}")), today)
            .unwrap();
    }
    lib.return_book(&book("B2"), today).unwrap();
    assert!(lib.borrow(&member("M1"), &book("B4"), today).is_ok());
}

#[test]
fn r6_member_with_overdue_book_cannot_borrow() {
    let mut lib = library();
    lib.borrow(&member("M1"), &book("B1"), d("2026-09-01"))
        .unwrap(); // 期限 09-15
                   // 期限当日はまだ延滞ではない
    assert!(lib
        .borrow(&member("M1"), &book("B2"), d("2026-09-15"))
        .is_ok());
    // 翌日からは延滞
    assert_eq!(
        lib.borrow(&member("M1"), &book("B3"), d("2026-09-16")),
        Err(LendingError::HasOverdue {
            member: member("M1"),
            book: book("B1")
        })
    );
    // 延滞している本を返せば、また借りられる
    lib.return_book(&book("B1"), d("2026-09-16")).unwrap();
    assert!(lib
        .borrow(&member("M1"), &book("B3"), d("2026-09-16"))
        .is_ok());
}

#[test]
fn r7_return_reports_overdue_days() {
    let mut lib = library();
    lib.borrow(&member("M1"), &book("B1"), d("2026-09-01"))
        .unwrap(); // 期限 09-15
    lib.borrow(&member("M1"), &book("B2"), d("2026-09-01"))
        .unwrap();

    let on_time = lib.return_book(&book("B1"), d("2026-09-15")).unwrap();
    assert_eq!(on_time.overdue_days, 0);
    assert_eq!(on_time.member, member("M1"));

    let late = lib.return_book(&book("B2"), d("2026-09-18")).unwrap();
    assert_eq!(late.overdue_days, 3);

    assert_eq!(lib.book(&book("B1")).unwrap().state, BookState::Available);
}

#[test]
fn r7_cannot_return_a_book_that_is_not_on_loan() {
    let mut lib = library();
    assert_eq!(
        lib.return_book(&book("B1"), d("2026-09-25")),
        Err(LendingError::NotOnLoan(book("B1")))
    );
}

#[test]
fn r8_overdue_list() {
    let mut lib = library();
    lib.borrow(&member("M1"), &book("B1"), d("2026-09-01"))
        .unwrap(); // 期限 09-15
    lib.borrow(&member("S1"), &book("B2"), d("2026-09-01"))
        .unwrap(); // 期限 10-01
    lib.borrow(&member("M1"), &book("B3"), d("2026-09-10"))
        .unwrap(); // 期限 09-24

    let ids = |today| -> Vec<String> {
        lib.overdue(d(today))
            .map(|(b, _)| b.id.to_string())
            .collect()
    };
    assert_eq!(ids("2026-09-15"), Vec::<String>::new());
    assert_eq!(ids("2026-09-16"), vec!["B1"]);
    assert_eq!(ids("2026-09-25"), vec!["B1", "B3"]);
    assert_eq!(ids("2026-10-02"), vec!["B1", "B2", "B3"]);
}

#[test]
fn r10_unknown_ids_are_rejected_without_changes() {
    let mut lib = library();
    let before = lib.clone();
    let today = d("2026-09-25");
    assert_eq!(
        lib.borrow(&member("X"), &book("B1"), today),
        Err(LendingError::UnknownMember(member("X")))
    );
    assert_eq!(
        lib.borrow(&member("M1"), &book("X"), today),
        Err(LendingError::UnknownBook(book("X")))
    );
    assert_eq!(
        lib.return_book(&book("X"), today),
        Err(LendingError::UnknownBook(book("X")))
    );
    assert_eq!(lib, before);
}

#[test]
fn r10_return_date_before_loan_is_rejected() {
    let mut lib = library();
    lib.borrow(&member("M1"), &book("B1"), d("2026-09-25"))
        .unwrap();
    assert_eq!(
        lib.return_book(&book("B1"), d("2026-09-24")),
        Err(LendingError::ReturnBeforeLoan {
            book: book("B1"),
            since: d("2026-09-25")
        })
    );
}
