//! Lesson 04-1: traitは「共通化」のためではない
//!
//! この演習は04-1のBad Exampleとは逆に、traitを導入する正当な理由が
//! 実際にある場面を実装する（2つの実装があり、両方を同じように扱う
//! 呼び出し側が実在する）。

pub trait PaymentMethod {
    /// 処理結果の説明を返す（本来は実際に課金処理をするが、ここでは
    /// テストしやすいよう文字列を返す設計にしている）。
    fn charge(&self, amount_cents: u64) -> String;
}

pub struct CreditCard {
    pub last4: String,
}

impl PaymentMethod for CreditCard {
    fn charge(&self, amount_cents: u64) -> String {
        todo!("例: \"charged N cents to card ending in XXXX\" のような文字列を返してください（format!を使う）")
    }
}

pub struct BankTransfer {
    pub account: String,
}

impl PaymentMethod for BankTransfer {
    fn charge(&self, amount_cents: u64) -> String {
        todo!("例: \"transferred N cents from account XXX\" のような文字列を返してください（format!を使う）")
    }
}

/// CreditCard・BankTransferのどちらでも同じように呼べる。
/// これが「traitを導入する正当な理由」——複数の実装を同じように扱う
/// 呼び出し側が実在する。
pub fn process_payment(method: &dyn PaymentMethod, amount_cents: u64) -> String {
    todo!("method.charge(amount_cents) を呼んで、その結果をそのまま返してください")
}
