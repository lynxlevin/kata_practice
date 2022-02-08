use shopping3::Basket;
use shopping3::Item;
use shopping3::TaxType;
use std::io;

fn main() {
    let mut basket = Basket::new();
    let apple = Item::new("apple".to_string(), 100.0, TaxType::Reduced);
    let wine = Item::new("wine".to_string(), 10000.0, TaxType::Normal);
    let items = [apple, wine];
    for item in items {
        println!(
            "How many {} do you want? {} yen for one",
            item.name(),
            item.price_with_tax()
        );
        let num = get_input();
        basket.add(item, num);
    }
    println!("{}", basket.sum());
}

fn get_input() -> f64 {
    let mut command = String::new();
    io::stdin()
        .read_line(&mut command)
        .expect("Failed to read line");
    let num: f64 = command.trim().parse().expect("Input not a number");
    num
}
