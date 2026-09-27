//! R9: 保存と読み戻し。

use final_lending::store::{load, save, StoreErrorKind};
use final_lending::{BookId, Date, LendingError, Library, MemberId, MemberKind};

fn sample() -> Library {
    let mut lib = Library::new();
    lib.add_member(
        MemberId::new("M1").unwrap(),
        "山田 花子",
        MemberKind::General,
    )
    .unwrap();
    lib.add_member(
        MemberId::new("S1").unwrap(),
        "タブ\tと\\記号",
        MemberKind::Staff,
    )
    .unwrap();
    lib.add_book(BookId::new("B1").unwrap(), "Rustの本")
        .unwrap();
    lib.add_book(BookId::new("B2").unwrap(), "改行\nを含む書名")
        .unwrap();
    lib.borrow(
        &MemberId::new("M1").unwrap(),
        &BookId::new("B2").unwrap(),
        "2026-09-01".parse::<Date>().unwrap(),
    )
    .unwrap();
    lib
}

#[test]
fn save_then_load_round_trips() {
    let lib = sample();
    assert_eq!(load(&save(&lib)).unwrap(), lib);
}

#[test]
fn saved_text_is_readable() {
    let text = save(&sample());
    assert_eq!(
        text,
        "# lending v2\n\
         member\tM1\tgeneral\t山田 花子\n\
         member\tS1\tstaff\tタブ\\tと\\\\記号\n\
         book\tB1\tRustの本\tavailable\n\
         book\tB2\t改行\\nを含む書名\tloan\tM1\t2026-09-01\t2026-09-15\n"
    );
}

#[test]
fn empty_library_round_trips() {
    assert_eq!(load(&save(&Library::new())).unwrap(), Library::new());
}

#[test]
fn loading_does_not_reapply_lending_rules() {
    // 一般の利用者が 4 冊借りている（上限は 3）。ルールが変わる前に保存されたデータを想定。
    // 読み込みでは、ルールではなく「データとして矛盾がないか」だけを確かめる。
    let text = "# lending v2\n\
                member\tM1\tgeneral\tA\n\
                book\tB1\tx\tloan\tM1\t2026-09-01\t2026-09-15\n\
                book\tB2\tx\tloan\tM1\t2026-09-01\t2026-09-15\n\
                book\tB3\tx\tloan\tM1\t2026-09-01\t2026-09-15\n\
                book\tB4\tx\tloan\tM1\t2026-09-01\t2026-09-15\n";
    let lib = load(text).unwrap();
    assert_eq!(lib.loans_of(&MemberId::new("M1").unwrap()).count(), 4);
}

fn error_at(text: &str) -> (usize, StoreErrorKind) {
    let e = load(text).unwrap_err();
    (e.line, e.kind)
}

#[test]
fn missing_or_wrong_header() {
    assert_eq!(error_at("").1, StoreErrorKind::UnsupportedFormat);
    assert_eq!(
        error_at("# lending v3\n").1,
        StoreErrorKind::UnsupportedFormat
    );
    assert_eq!(
        error_at("member\tM1\tgeneral\tA\n").1,
        StoreErrorKind::UnsupportedFormat
    );
}

#[test]
fn broken_lines_report_line_numbers() {
    let (line, kind) = error_at("# lending v2\n\nshelf\tS1\n");
    assert_eq!(line, 3);
    assert_eq!(kind, StoreErrorKind::UnknownRecord("shelf".to_string()));

    let (line, kind) = error_at("# lending v2\nmember\tM1\tgeneral\n");
    assert_eq!(line, 2);
    assert_eq!(kind, StoreErrorKind::WrongFieldCount);

    let (_, kind) = error_at("# lending v2\nmember\tM1\tvisitor\tA\n");
    assert!(matches!(kind, StoreErrorKind::InvalidField(_)));

    let (_, kind) = error_at(
        "# lending v2\nmember\tM1\tgeneral\tA\nbook\tB1\tx\tloan\tM1\t2026-02-30\t2026-03-01\n",
    );
    assert!(matches!(kind, StoreErrorKind::InvalidField(_)));
}

#[test]
fn inconsistent_data_is_rejected() {
    // 存在しない利用者への貸出
    let (line, kind) = error_at("# lending v2\nbook\tB1\tx\tloan\tM9\t2026-09-01\t2026-09-15\n");
    assert_eq!(line, 2);
    assert_eq!(
        kind,
        StoreErrorKind::Inconsistent(LendingError::UnknownMember(MemberId::new("M9").unwrap()))
    );
    // ID の重複
    let (line, _) = error_at("# lending v2\nbook\tB1\tx\tavailable\nbook\tB1\ty\tavailable\n");
    assert_eq!(line, 3);
}

#[test]
fn crlf_files_are_accepted() {
    let text = save(&sample()).replace('\n', "\r\n");
    assert_eq!(load(&text).unwrap(), sample());
}

#[test]
fn v1_files_are_read_with_general_members() {
    // 種別が無かった頃（v1）のファイル。全員「一般」として読む
    let text = "# lending v1\n\
                member\tM1\t山田\n\
                book\tB1\tx\tloan\tM1\t2026-09-01\t2026-09-15\n";
    let lib = load(text).unwrap();
    let m = lib.member(&MemberId::new("M1").unwrap()).unwrap();
    assert_eq!(m.kind, MemberKind::General);
    assert_eq!(m.name, "山田");
    // 保存し直すと v2 になる
    assert!(save(&lib).starts_with("# lending v2\nmember\tM1\tgeneral\t山田\n"));
}

#[test]
fn v2_member_line_needs_kind() {
    let (line, kind) = error_at("# lending v2\nmember\tM1\t山田\n");
    assert_eq!(line, 2);
    assert_eq!(kind, StoreErrorKind::WrongFieldCount);
}
