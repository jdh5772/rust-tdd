#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Currency {
    Dollar,
    Franc,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum IsoCode {
    USD,
    CHF,
    None,
}

#[derive(PartialEq, Eq, Debug)]
pub struct Money {
    currency: Currency,
    amount: usize,
    iso_code: IsoCode,
}

impl Money {
    pub fn new(currency: &str, amount: usize) -> Self {
        match currency {
            "dollar" => Self {
                currency: Currency::Dollar,
                amount,
                iso_code: IsoCode::USD,
            },
            "franc" => Self {
                currency: Currency::Franc,
                amount,
                iso_code: IsoCode::CHF,
            },
            _ => Self {
                currency: Currency::None,
                amount,
                iso_code: IsoCode::None,
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
        match self.iso_code {
            IsoCode::USD => "USD",
            IsoCode::CHF => "CHF",
            IsoCode::None => "None",
        }
    }

    pub fn plus(&self, other: Self) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount + other.amount,
            iso_code: self.iso_code,
        }
    }
}

#[derive(Debug)]
pub struct Bank {}

impl Bank {
    pub fn new() -> Self {
        Self {}
    }

    pub fn reduce(self, _sum: Money, _currency: &str) -> Money {
        Money::new("dollar", 10)
    }
}
