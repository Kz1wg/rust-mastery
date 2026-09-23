//! Lesson 14-3: procedural macro 入門
//!
//! derive マクロは「struct の定義（トークンの列）を受け取って、追加するコード（トークンの列）を返す関数」。
//! 入力を読む部分（struct_name / field_names）は用意してある。
//! 実装するのは、生成するコードを組み立てる generate_code。

use proc_macro::{Delimiter, Spacing, TokenStream, TokenTree};

#[proc_macro_derive(Describe)]
pub fn derive_describe(input: TokenStream) -> TokenStream {
    let tokens: Vec<TokenTree> = input.into_iter().collect();
    let name = struct_name(&tokens);
    let fields = field_names(&tokens);

    generate_code(&name, &fields)
        .parse()
        .expect("生成したコードが Rust として正しくありません")
}

/// 生成するコード（文字列）を組み立てる。
///
/// 例えば name が "User"、fields が ["name", "age"] なら、次のようなコードを作る:
///
/// ```text
/// impl User {
///     pub fn type_name() -> &'static str { "User" }
///     pub fn field_names() -> Vec<&'static str> { vec!["name", "age"] }
/// }
/// ```
///
/// format! の中では、文字としての波かっこは 2重（{{ と }}）にする必要がある。
fn generate_code(name: &str, fields: &[String]) -> String {
    // フィールド名を "name", "age" のように、引用符付きのカンマ区切りにする
    let quoted: Vec<String> = fields.iter().map(|f| format!("\"{f}\"")).collect();
    let field_list = quoted.join(", ");

    format!(
        "impl {name} {{
            pub fn type_name() -> &'static str {{ \"{name}\" }}
            pub fn field_names() -> Vec<&'static str> {{ vec![{field_list}] }}
        }}"
    )
}

/// `struct` の直後の名前を探す。
fn struct_name(tokens: &[TokenTree]) -> String {
    for pair in tokens.windows(2) {
        if let (TokenTree::Ident(keyword), TokenTree::Ident(name)) = (&pair[0], &pair[1]) {
            if keyword.to_string() == "struct" {
                return name.to_string();
            }
        }
    }
    panic!("#[derive(Describe)] は struct にだけ使えます");
}

/// `{ ... }` の中から、フィールド名（直後に単独の `:` が続く名前）を順に取り出す。
/// `std::string::String` のような `::` は、`:` が2つ連なっているので区別できる。
fn field_names(tokens: &[TokenTree]) -> Vec<String> {
    let body = tokens.iter().find_map(|t| match t {
        TokenTree::Group(g) if g.delimiter() == Delimiter::Brace => Some(g.stream()),
        _ => None,
    });
    let Some(body) = body else {
        return Vec::new(); // フィールドの無い struct（struct Unit; など）
    };

    let inner: Vec<TokenTree> = body.into_iter().collect();
    let mut names = Vec::new();
    for (i, token) in inner.iter().enumerate() {
        let TokenTree::Ident(ident) = token else {
            continue;
        };
        let next_is_single_colon = matches!(
            inner.get(i + 1),
            Some(TokenTree::Punct(p)) if p.as_char() == ':' && p.spacing() == Spacing::Alone
        );
        let prev_is_colon = matches!(
            i.checked_sub(1).and_then(|j| inner.get(j)),
            Some(TokenTree::Punct(p)) if p.as_char() == ':'
        );
        if next_is_single_colon && !prev_is_colon {
            names.push(ident.to_string());
        }
    }
    names
}
