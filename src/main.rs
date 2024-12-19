mod lmsr; // Import the module

fn main() {
    let mut market = lmsr::LMSR::new(10.0, 3); // Initialize LMSR with 3 outcomes and b=10

    println!("Initial cost: {}", market.cost());

    println!("Buying shares in outcome 1...");
    market.buy(1, 5.0);

    println!("Updated cost: {}", market.cost());
    println!("Price of outcome 1: {}", market.price(1));
}
