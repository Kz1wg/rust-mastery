pub trait PaymentMethod {
    fn charge(&self, amount_cents: u64) -> String;
}

pub struct CreditCard {
    pub last4: String,
}

impl PaymentMethod for CreditCard {
    fn charge(&self, amount_cents: u64) -> String {
        format!(
            "charged {amount_cents} cents to card ending in {}",
            self.last4
        )
    }
}

pub struct BankTransfer {
    pub account: String,
}

impl PaymentMethod for BankTransfer {
    fn charge(&self, amount_cents: u64) -> String {
        format!(
            "transferred {amount_cents} cents from account {}",
            self.account
        )
    }
}

pub fn process_payment(method: &dyn PaymentMethod, amount_cents: u64) -> String {
    method.charge(amount_cents)
}
