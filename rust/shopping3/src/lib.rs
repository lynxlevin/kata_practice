pub enum TaxType {
    Normal,
    Reduced,
}

pub struct Item {
    name: String,
    price: f64,
    tax_type: TaxType,
}

impl Item {
    pub fn new(name: String, price: f64, tax_type: TaxType) -> Item {
        Item {
            name,
            price,
            tax_type,
        }
    }

    pub fn price_with_tax(&self) -> f64 {
        self.price * self.tax_rate()
    }

    fn tax_rate(&self) -> f64 {
        match &self.tax_type {
            TaxType::Normal => 1.1,
            TaxType::Reduced => 1.08,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub struct Basket {
    items: Vec<(Item, f64)>,
}

impl Basket {
    pub fn new() -> Basket {
        Basket { items: vec![] }
    }

    pub fn add(&mut self, item: Item, num: f64) {
        self.items.push((item, num));
    }

    pub fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for item in &self.items {
            sum += item.0.price_with_tax() * item.1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_price_with_reduced_tax() {
        let apple = Item::new("apple".to_string(), 100.0, TaxType::Reduced);
        assert_eq!(apple.price_with_tax(), 108.0);
    }

    #[test]
    fn test_price_with_normal_tax() {
        let wine = Item::new("wine".to_string(), 10000.0, TaxType::Normal);
        assert_eq!(wine.price_with_tax(), 11000.0);
    }

    #[test]
    fn test_basket_sum() {
        let mut basket = Basket::new();
        let apple = Item::new("apple".to_string(), 100.0, TaxType::Reduced);
        let wine = Item::new("wine".to_string(), 10000.0, TaxType::Normal);
        basket.add(apple, 3.0);
        basket.add(wine, 2.0);
        assert_eq!(basket.sum(), 22324.0);
    }

    #[test]
    fn test_get_item_name() {
        let apple = Item::new("apple".to_string(), 100.0, TaxType::Reduced);
        assert_eq!(apple.name(), "apple");
    }
}
