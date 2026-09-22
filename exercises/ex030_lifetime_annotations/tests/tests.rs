use ex030_lifetime_annotations::{first, longest};

#[test]
fn longest_picks_the_longer_one() {
    assert_eq!(longest("hello", "hi"), "hello");
    assert_eq!(longest("hi", "hello"), "hello");
}

#[test]
fn longest_prefers_a_on_tie() {
    assert_eq!(longest("ab", "cd"), "ab");
}

#[test]
fn first_always_returns_a() {
    assert_eq!(first("keep", "longer string"), "keep");
}

/// first の戻り値は b を借りていないので、b が破棄された後も使える。
/// first のシグネチャが longest と同じ（両方 'a）だったら、この関数はコンパイルできない。
#[test]
fn first_result_outlives_b() {
    let a = String::from("keep me");
    let result;
    {
        let b = String::from("temporary");
        result = first(&a, &b);
    }
    assert_eq!(result, "keep me");
}
