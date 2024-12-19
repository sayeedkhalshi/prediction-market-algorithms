pub struct CPMM {
    pub reserve_x: f64, // Reserve of token X
    pub reserve_y: f64, // Reserve of token Y
}

impl CPMM {
    // Create a new CPMM instance
    pub fn new(reserve_x: f64, reserve_y: f64) -> Self {
        Self { reserve_x, reserve_y }
    }

    // Calculate current price of token X in terms of token Y
    pub fn price_x(&self) -> f64 {
        self.reserve_y / self.reserve_x
    }

    // Swap token X for token Y
    pub fn swap_x_for_y(&mut self, delta_x: f64) -> f64 {
        let new_reserve_x = self.reserve_x + delta_x;
        let new_reserve_y = (self.reserve_x * self.reserve_y) / new_reserve_x;
        let delta_y = self.reserve_y - new_reserve_y;

        self.reserve_x = new_reserve_x;
        self.reserve_y = new_reserve_y;

        delta_y
    }

    // Swap token Y for token X
    pub fn swap_y_for_x(&mut self, delta_y: f64) -> f64 {
        let new_reserve_y = self.reserve_y + delta_y;
        let new_reserve_x = (self.reserve_x * self.reserve_y) / new_reserve_y;
        let delta_x = self.reserve_x - new_reserve_x;

        self.reserve_x = new_reserve_x;
        self.reserve_y = new_reserve_y;

        delta_x
    }
}
