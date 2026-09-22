use ex013_trait_purpose::{process_payment, BankTransfer, CreditCard, PaymentMethod};

#[test]
fn credit_card_charge_mentions_last4() {
    let card = CreditCard {
        last4: "4242".to_string(),
    };
    let result = card.charge(500);
    assert!(result.contains("4242"));
    assert!(result.contains("500"));
}

#[test]
fn bank_transfer_charge_mentions_account() {
    let transfer = BankTransfer {
        account: "ACC-001".to_string(),
    };
    let result = transfer.charge(1000);
    assert!(result.contains("ACC-001"));
    assert!(result.contains("1000"));
}

#[test]
fn process_payment_works_with_credit_card() {
    let card = CreditCard {
        last4: "1111".to_string(),
    };
    let result = process_payment(&card, 250);
    assert!(result.contains("1111"));
}

#[test]
fn process_payment_works_with_bank_transfer() {
    let transfer = BankTransfer {
        account: "ACC-999".to_string(),
    };
    let result = process_payment(&transfer, 750);
    assert!(result.contains("ACC-999"));
}

/// process_payment が &dyn PaymentMethod を受けるおかげで、異なる実装を
/// 同じ関数呼び出しで扱えることを確認する。
#[test]
fn process_payment_treats_different_implementors_uniformly() {
    let methods: Vec<Box<dyn PaymentMethod>> = vec![
        Box::new(CreditCard {
            last4: "0000".to_string(),
        }),
        Box::new(BankTransfer {
            account: "ACC-X".to_string(),
        }),
    ];
    for method in &methods {
        let result = process_payment(method.as_ref(), 100);
        assert!(!result.is_empty());
    }
}
