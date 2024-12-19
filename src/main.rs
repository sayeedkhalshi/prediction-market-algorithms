mod lmsr;
mod cpmm;

fn main() {
    // Using LMSR
    let mut market = lmsr::LMSR::new(10.0, 3);
    println!("Initial LMSR cost: {}", market.cost());

    market.buy(1, 5.0);
    println!("Updated LMSR cost: {}", market.cost());
    println!("LMSR price of outcome 1: {}", market.price(1));

    // Using CPMM
    let mut pool = cpmm::CPMM::new(100.0, 200.0); // 100 X and 200 Y
    println!("Initial CPMM price of X: {}", pool.price_x());

    let delta_y = pool.swap_x_for_y(10.0); // Swap 10 X for Y
    println!("Swapped 10 X for {} Y", delta_y);
    println!("Updated CPMM price of X: {}", pool.price_x());

    let delta_x = pool.swap_y_for_x(20.0); // Swap 20 Y for X
    println!("Swapped 20 Y for {} X", delta_x);
    println!("Updated CPMM price of X: {}", pool.price_x());
}
