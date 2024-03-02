use chrono::Month;

use crate::domain::{
    aggregate::{invoice::Invoice, value_object::recipient::Recipient},
    interface::invoice_repository_interface::InvoiceRepositoryInterface,
};

pub struct CreateInvoiceUseCase<T>
where
    T: InvoiceRepositoryInterface,
{
    repository: T,
}

impl<T> CreateInvoiceUseCase<T>
where
    T: InvoiceRepositoryInterface,
{
    pub fn new(repository: T) -> Self {
        CreateInvoiceUseCase { repository }
    }

    pub fn execute(&self) {
        let month = Month::try_from(12).unwrap();
        let recipient = Recipient::from_str("husband").unwrap();
        let invoce = Invoice::new(month, recipient);
        let _ = self.repository.create(&invoce);
    }
}
