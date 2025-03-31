mod math_operations;

use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=100);
    println!("Random Number: {}", random_number);

    let sum = math_operations::add(5, 7);
    let product = math_operations::multiply(6, 8);

    println!("Sum: {}", sum);
    println!("Product: {}", product);
}
