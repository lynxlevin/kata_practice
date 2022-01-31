pub enum TaxType {
    Normal,
    Reduced,
}

pub struct Basket {
    pub items: Vec<(Item, u64)>,
}

impl Basket {
    pub fn sum(&self) -> u64 {
        let mut sum = 0;
        for item in &self.items {
            sum += item.0.taxed_price() * item.1;
        }
        sum
    }
}

pub struct Item {
    pub name: String,
    pub price: u64,
    pub tax_type: TaxType,
}

impl Item {
    fn taxed_price(&self) -> u64 {
        self.price * self.tax_rate() / 100
    }

    fn tax_rate(&self) -> u64 {
        match self.tax_type {
            TaxType::Normal => 110,
            TaxType::Reduced => 108,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_taxed_price_reduced_tax() {
        let apple = Item {
            name: String::from("apple"),
            price: 100,
            tax_type: TaxType::Reduced,
        };
        assert_eq!(apple.taxed_price(), 108);
    }

    #[test]
    fn test_taxed_price_normal_tax() {
        let soap = Item {
            name: String::from("soap"),
            price: 300,
            tax_type: TaxType::Normal,
        };
        assert_eq!(soap.taxed_price(), 330);
    }

    #[test]
    fn test_basket_sum() {
        let apple = Item {
            name: String::from("apple"),
            price: 100,
            tax_type: TaxType::Reduced,
        };
        let soap = Item {
            name: String::from("soap"),
            price: 300,
            tax_type: TaxType::Normal,
        };
        let basket = Basket {
            items: vec![(apple, 3), (soap, 5)],
        };
        assert_eq!(basket.sum(), 1974);
    }
}
