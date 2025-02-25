pub trait Money: Sized {
    fn new(amount: usize) -> Self;
    fn amount(&self) -> usize;
    fn multiple(&self, n: usize) -> Self {
        Self::new(self.amount() * n)
    }
    fn equals(&self, other: Self) -> bool {
        self.amount() == other.amount()
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Dollar {
    amount: usize,
}

impl Money for Dollar {
    fn new(amount: usize) -> Self {
        Self { amount }
    }
    fn amount(&self) -> usize {
        self.amount
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Franc {
    amount: usize,
}

impl Money for Franc {
    fn new(amount: usize) -> Self {
        Self { amount }
    }
    fn amount(&self) -> usize {
        self.amount
    }
}
