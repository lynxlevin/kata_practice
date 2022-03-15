#[derive(Debug, PartialEq, Clone, Copy)]
struct Item {
    price: f64,
    tax_rate: TaxRate,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum TaxRate {
    Normal,
    Reduced,
}

impl Item {
    fn new(price: f64, tax_rate: TaxRate) -> Item {
        Item { price, tax_rate }
    }

    fn price_ati(&self) -> f64 {
        self.price * self.get_tax_rate()
    }

    fn get_tax_rate(&self) -> f64 {
        match self.tax_rate {
            TaxRate::Normal => 1.1,
            TaxRate::Reduced => 1.08,
        }
    }
}

struct ShoppingBag {
    items: Vec<(Item, f64)>,
}

impl ShoppingBag {
    fn new() -> ShoppingBag {
        ShoppingBag { items: vec![] }
    }

    fn add(&mut self, item: Item, num: f64) {
        self.items.push((item, num));
    }

    fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for item in &self.items {
            sum += item.0.price_ati() * item.1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_returns_value_with_reduced_tax() {
        let apple = Item::new(100.0, TaxRate::Reduced);
        let result = apple.price_ati();
        assert_eq!(result, 108.0);
    }

    #[test]
    fn test_item_returns_value_with_normal_tax() {
        let pc = Item::new(10000.0, TaxRate::Normal);
        let result = pc.price_ati();
        assert_eq!(result, 11000.0);
    }

    #[test]
    fn test_shopping_basket_add() {
        let apple = Item::new(100.0, TaxRate::Reduced);
        let pc = Item::new(10000.0, TaxRate::Normal);
        let mut bag = ShoppingBag::new();
        bag.add(apple, 3.0);
        bag.add(pc, 1.0);
        assert_eq!(bag.items[0].0, apple);
        assert_eq!(bag.items[0].1, 3.0);
    }

    #[test]
    fn test_shopping_basket_sum() {
        let apple = Item::new(100.0, TaxRate::Reduced);
        let pc = Item::new(10000.0, TaxRate::Normal);
        let mut bag = ShoppingBag::new();
        bag.add(apple, 3.0);
        bag.add(pc, 1.0);
        let result = bag.sum();
        assert_eq!(result, 11324.0);
    }
}
