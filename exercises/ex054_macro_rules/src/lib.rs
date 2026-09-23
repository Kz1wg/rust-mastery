//! Lesson 14-2: declarative macro
//!
//! - hashmap!: key => value の組を繰り返し受け取って HashMap を作る
//! - impl_unit!: 単位ごとに newtype と Display 実装をまとめて生成する（項目の生成）

/// `key => value` の組から HashMap を作る。最後のカンマがあってもよい。
///
/// ```
/// use ex054_macro_rules::hashmap;
/// use std::collections::HashMap;
/// let m: HashMap<&str, i32> = hashmap! { "a" => 1, "b" => 2, };
/// assert_eq!(m["b"], 2);
/// ```
#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        $(
            let _ = (&$key, &$value);
        )*
        todo!("HashMap を作り、繰り返しの中で key と value を insert して返してください")
    }};
}

/// 単位ごとに、f64 を包む newtype と、記号付きで表示する Display 実装を作る。
///
/// impl_unit!(Meters, "m") は次を生成する:
/// - pub struct Meters(pub f64);
/// - impl Display for Meters（"3m" のように表示する）
#[macro_export]
macro_rules! impl_unit {
    ($($name:ident, $symbol:literal);* $(;)?) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct $name(pub f64);

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    let _ = (f, self.0, $symbol);
                    todo!("self.0 と記号を続けて書き込んでください（write! を使う）")
                }
            }
        )*
    };
}

impl_unit!(Meters, "m"; Seconds, "s"; Grams, "g");
