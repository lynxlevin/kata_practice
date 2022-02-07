use shopping2::{Item, ShoppingCart, TaxType};

fn main() {
    let apple = Item {
        price: 100.0,
        tax_type: TaxType::Reduced,
    };
    let wine = Item {
        price: 10000.0,
        tax_type: TaxType::Normal,
    };
    let cart = ShoppingCart {
        items: vec![(apple, 1.0), (wine, 2.0)],
    };
    println!("{}", cart.sum());
}
