pub struct Dollar {
    pub amount: usize,
}

impl Dollar {
    pub fn new(amount: usize) -> Self {
        Self { amount }
    }
    pub fn multiple(&self, n: usize) -> Self {
        Self {
            amount: self.amount * n,
        }
    }
    pub fn equals(&self, dollar: Dollar) -> bool {
        self.amount == dollar.amount
    }
}
