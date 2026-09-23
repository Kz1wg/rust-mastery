/// ```
/// use ex054_macro_rules::hashmap;
/// use std::collections::HashMap;
/// let m: HashMap<&str, i32> = hashmap! { "a" => 1, "b" => 2, };
/// assert_eq!(m["b"], 2);
/// ```
#[macro_export]
macro_rules! hashmap {
    ($($key:expr => $value:expr),* $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

#[macro_export]
macro_rules! impl_unit {
    ($($name:ident, $symbol:literal);* $(;)?) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq)]
            pub struct $name(pub f64);

            impl ::std::fmt::Display for $name {
                fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    write!(f, "{}{}", self.0, $symbol)
                }
            }
        )*
    };
}

impl_unit!(Meters, "m"; Seconds, "s"; Grams, "g");
