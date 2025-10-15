#[derive(Debug, Clone, Copy)]
pub struct Money {
    pub currency: Currency,
    pub amount: f64,
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.currency.format(self.amount, f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Currency {
    EUR,
    USD,
}

impl Currency {
    pub fn format(&self, amount: f64, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::EUR => write!(f, "{:.2} €", amount),
            Currency::USD => write!(f, "$ {:.2}", amount),
        }
    }
}
