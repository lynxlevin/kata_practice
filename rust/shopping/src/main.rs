// This took 35mins.

use shopping::Basket;
use shopping::Item;
use shopping::TaxType;

fn main() {
    let apple = Item {
        name: String::from("apple"),
        price: 100,
        tax_type: TaxType::Reduced,
    };

    let soap = Item {
        name: String::from("soap"),
        price: 350,
        tax_type: TaxType::Normal,
    };

    let wine = Item {
        name: String::from("wine"),
        price: 5600,
        tax_type: TaxType::Normal,
    };

    let grape = Item {
        name: String::from("grape"),
        price: 800,
        tax_type: TaxType::Reduced,
    };

    let basket = Basket {
        items: vec![(apple, 3), (soap, 5), (wine, 1), (grape, 0)],
    };

    println!("{}", basket.sum());
}
