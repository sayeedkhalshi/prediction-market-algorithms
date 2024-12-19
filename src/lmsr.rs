pub struct LMSR {
    pub b: f64,            // Liquidity parameter
    pub quantities: Vec<f64>, // Quantities of shares for each outcome
}

impl LMSR {
    pub fn new(b: f64, outcomes: usize) -> Self {
        Self {
            b,
            quantities: vec![0.0; outcomes],
        }
    }

    // Calculate cost function
    pub fn cost(&self) -> f64 {
        let sum_exp: f64 = self.quantities.iter().map(|q| (q / self.b).exp()).sum();
        self.b * sum_exp.ln()
    }

    // Buy shares of a specific outcome
    pub fn buy(&mut self, outcome: usize, amount: f64) -> f64 {
        self.quantities[outcome] += amount;
        self.cost()
    }

    // Calculate price of an outcome
    pub fn price(&self, outcome: usize) -> f64 {
        let exp_vals: Vec<f64> = self.quantities.iter().map(|q| (q / self.b).exp()).collect();
        exp_vals[outcome] / exp_vals.iter().sum::<f64>()
    }
}
