#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Currency {
    Dollar,
    Franc,
    None,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Money {
    currency: Currency,
    amount: usize,
    iso_code: &'static str,
}

impl Money {
    pub fn new(currency: &str, amount: usize) -> Self {
        match currency {
            "dollar" => Self {
                currency: Currency::Dollar,
                amount,
                iso_code: "USD",
            },
            "franc" => Self {
                currency: Currency::Franc,
                amount,
                iso_code: "CHF",
            },
            _ => Self {
                currency: Currency::None,
                amount: 0,
                iso_code: "",
            },
        }
    }

    pub fn times(&self, n: usize) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount * n,
            iso_code: self.iso_code,
        }
    }

    pub fn equals(&self, other: Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }

    pub fn currency(&self) -> &str {
        self.iso_code
    }
}
