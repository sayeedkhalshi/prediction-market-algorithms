pub struct ABLMSR {
    pub b: f64,                // Liquidity parameter
    pub quantities: Vec<f64>,  // Quantities of shares for each outcome
    pub b_min: f64,            // Minimum value of b
    pub b_max: f64,            // Maximum value of b
    pub sensitivity: f64,      // Sensitivity for adjusting b
}

impl ABLMSR {
    // Create a new ABLMSR instance
    pub fn new(b: f64, b_min: f64, b_max: f64, outcomes: usize, sensitivity: f64) -> Self {
        Self {
            b,
            quantities: vec![0.0; outcomes],
            b_min,
            b_max,
            sensitivity,
        }
    }

    // Calculate the cost function
    pub fn cost(&self) -> f64 {
        let sum_exp: f64 = self.quantities.iter().map(|q| (q / self.b).exp()).sum();
        self.b * sum_exp.ln()
    }

    // Adjust b dynamically based on market activity
    pub fn adjust_b(&mut self) {
        let total_quantity: f64 = self.quantities.iter().sum();
        let adjustment = self.sensitivity * total_quantity;

        self.b = (self.b + adjustment)
            .clamp(self.b_min, self.b_max); // Ensure b remains within bounds
    }

    // Buy shares for a specific outcome
    pub fn buy(&mut self, outcome: usize, amount: f64) -> f64 {
        self.quantities[outcome] += amount;
        let cost = self.cost();
        self.adjust_b(); // Adjust b after the trade
        cost
    }

    // Get the price of an outcome
    pub fn price(&self, outcome: usize) -> f64 {
        let exp_vals: Vec<f64> = self.quantities.iter().map(|q| (q / self.b).exp()).collect();
        exp_vals[outcome] / exp_vals.iter().sum::<f64>()
    }
}
