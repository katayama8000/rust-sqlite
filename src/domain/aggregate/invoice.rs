use chrono::Month;

use super::value_object::{invoice_id::InvoiceId, payment::Payment, recipient::Recipient};

// 請求集約の構造体
#[derive(Debug)]
pub struct Invoice {
    id: InvoiceId,
    month: Month,
    recipient: Recipient,
    payments: Vec<Payment>, // 支払いのリスト
    is_paid: bool,
}

impl Invoice {
    // 新しい請求を作成するための関連関数
    pub fn new(month: Month, recipient: Recipient) -> Self {
        let id = InvoiceId::gen();
        Self {
            id,
            month,
            recipient,
            payments: Vec::new(),
            is_paid: false,
        }
    }

    // 支払いを追加するメソッド
    fn add_payment(&mut self, title: String, amount: u32, quantity: u32) {
        let payment = Payment::new(title, amount, quantity);
        self.payments.push(payment);
    }

    // 支払い完了をマークするメソッド
    fn mark_as_paid(&mut self) {
        self.is_paid = true;
    }

    // 支払い合計額を計算するメソッド
    fn total_amount(&self) -> u32 {
        self.payments.iter().map(|payment| payment.total()).sum()
    }
}
