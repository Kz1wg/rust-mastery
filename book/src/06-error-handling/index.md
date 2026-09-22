# 06 Error Handling

`Result` は「失敗するかもしれない」ことを型で表す仕組みです。
この章の中心的な問いは、**「呼び出し側は、このエラーを見て何を変える必要があるのか」**です。

## この章の到達目標

- `Result` の `Err` 型を、呼び出し側が実際に区別したい粒度で設計できる
- 独自の `enum Error` と `Box<dyn Error>` を、状況に応じて使い分けられる
- `?` 演算子が何をしているか（`From` による自動変換）を説明できる
- ライブラリとアプリケーションで、エラー設計の責任がどう違うか説明できる

## Lesson一覧

| Lesson | 問い |
| --- | --- |
| [06-1 `Result` の型設計](06-1-result-design.md) | 失敗をどの粒度で型にするか |
| [06-2 独自Error型](06-2-custom-error-types.md) | `enum Error` と `Box<dyn Error>` の使い分け |
| [06-3 error propagation](06-3-error-propagation.md) | `?` と `From` は何をしているか |
| [06-4 libraryとapplicationのerror設計](06-4-library-vs-application.md) | 誰がエラーを処理する責任を負うか |
