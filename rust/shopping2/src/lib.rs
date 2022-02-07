pub enum TaxType {
    Normal,
    Reduced,
}

pub struct Item {
    pub price: f64,
    pub tax_type: TaxType,
}

impl Item {
    fn taxed_price(&self) -> f64 {
        self.price * self.tax_rate()
    }

    fn tax_rate(&self) -> f64 {
        match self.tax_type {
            TaxType::Normal => 1.1,
            TaxType::Reduced => 1.08,
        }
    }
}

pub struct ShoppingCart {
    pub items: Vec<(Item, f64)>,
}

impl ShoppingCart {
    pub fn sum(&self) -> f64 {
        let mut sum = 0.0;
        for item in &self.items {
            sum += item.0.taxed_price() * item.1;
        }
        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reduced_tax() {
        let apple = Item {
            price: 100.0,
            tax_type: TaxType::Reduced,
        };
        assert_eq!(apple.taxed_price(), 108.0);
    }

    #[test]
    fn test_normal_tax() {
        let wine = Item {
            price: 10000.0,
            tax_type: TaxType::Normal,
        };
        assert_eq!(wine.taxed_price(), 11000.0);
    }

    #[test]
    fn test_sum() {
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
        assert_eq!(cart.sum(), 22108.0);
    }
}
