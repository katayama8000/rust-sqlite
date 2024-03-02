#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvoiceId(usize);

impl InvoiceId {
    pub fn gen() -> Self {
        Self(rand::random::<usize>())
    }
}
