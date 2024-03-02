#[derive(Debug)]
pub enum Recipient {
    Husband,
    Wife,
}

impl Recipient {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "husband" => Ok(Self::Husband),
            "wife" => Ok(Self::Wife),
            _ => Err(format!("Invalid recipient: {}", s)),
        }
    }
}
