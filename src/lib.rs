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
}

impl Money {
    pub fn new(currency: &str, amount: usize) -> Self {
        match currency {
            "dollar" => Self {
                currency: Currency::Dollar,
                amount,
            },
            "franc" => Self {
                currency: Currency::Franc,
                amount,
            },
            _ => Self {
                currency: Currency::None,
                amount: 0,
            },
        }
    }

    pub fn times(&self, n: usize) -> Self {
        Self {
            currency: self.currency,
            amount: self.amount * n,
        }
    }

    pub fn equals(&self, other: Self) -> bool {
        self.currency == other.currency && self.amount == other.amount
    }
}
