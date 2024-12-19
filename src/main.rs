mod lmsr;
mod cpmm;
mod ablmsr;

fn main() {
    // Adaptive Bounded LMSR
    let mut market = ablmsr::ABLMSR::new(10.0, 5.0, 20.0, 3, 0.1); // Initial b=10, b_min=5, b_max=20, 3 outcomes

    println!("Initial ABLMSR cost: {}", market.cost());
    println!("Initial b: {}", market.b);

    println!("Buying shares in outcome 1...");
    market.buy(1, 5.0);

    println!("Updated ABLMSR cost: {}", market.cost());
    println!("Updated b: {}", market.b);
    println!("Price of outcome 1: {}", market.price(1));

    println!("Buying shares in outcome 2...");
    market.buy(2, 10.0);

    println!("Updated ABLMSR cost: {}", market.cost());
    println!("Updated b: {}", market.b);
    println!("Price of outcome 2: {}", market.price(2));
}
