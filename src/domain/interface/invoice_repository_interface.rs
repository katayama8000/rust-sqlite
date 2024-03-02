use anyhow::Error;

use crate::domain::aggregate::{invoice::Invoice, value_object::invoice_id::InvoiceId};

pub trait InvoiceRepositoryInterface {
    // fn find_circle_by_id(&self, circle_id: &CircleId) -> Result<Circle, Error>;
    // fn create(&self, circle: &Circle) -> Result<(), Error>;
    // fn update(&self, circle: &Circle) -> Result<Circle, Error>;
    // fn delete(&self, circle: &Circle) -> Result<(), Error>;

    fn find_by_id(&self, id: &InvoiceId) -> Result<Invoice, Error>;
    fn find_all(&self) -> Result<Vec<Invoice>, Error>;
    fn create(&self, invoice: &Invoice) -> Result<(), Error>;
    fn update(&self, invoice: &Invoice) -> Result<Invoice, Error>;
    fn delete(&self, invoice: &Invoice) -> Result<(), Error>;
}
