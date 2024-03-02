use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Payment {
    title: String,
    amount: u32,
    quantity: u32,
    createdAt: DateTime<Utc>,
}

impl Payment {
    pub fn new(title: String, amount: u32, quantity: u32) -> Self {
        Self {
            title,
            amount,
            quantity,
            createdAt: chrono::Utc::now(),
        }
    }

    pub fn total(&self) -> u32 {
        self.amount * self.quantity
    }
}
