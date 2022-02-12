pub mod address_book {
    use crate::address::address::Address;
    use chrono::NaiveDate;

    #[derive(Debug, PartialEq)]
    pub struct AddressBook {
        addresses: Vec<Address>,
    }

    impl AddressBook {
        pub fn new() -> AddressBook {
            AddressBook { addresses: vec![] }
        }

        pub fn add(&mut self, address: Address) {
            self.addresses.push(address);
        }

        fn is_empty(&self) -> bool {
            self.addresses.len() == 0
        }

        pub fn get_birthday_people(&self, date: NaiveDate) -> Option<AddressBook> {
            let mut result = AddressBook::new();
            for address in &self.addresses {
                if address.is_birthday(date) {
                    result.add(address.clone());
                }
            }
            match result.is_empty() {
                true => None,
                false => Some(result),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn create_address_book() -> AddressBook {
            let mut address_book = AddressBook::new();
            address_book.add(Address::new(
                "Doe",
                "John",
                "1982/10/08",
                "john.doe@foobar.com",
            ));
            address_book.add(Address::new(
                "Ann",
                "Mary",
                "1975/09/11",
                "mary.ann@foobar.com",
            ));
            address_book
        }

        #[test]
        fn test_is_empty() {
            let address_book = AddressBook::new();
            assert!(address_book.is_empty());

            let address_book = create_address_book();
            assert!(!address_book.is_empty());
        }

        #[test]
        fn test_get_birthday_people() {
            let address_book = create_address_book();

            let john = Address::new("Doe", "John", "1982/10/08", "john.doe@foobar.com");
            let date = john.date_of_birth();
            let mut expected = AddressBook::new();
            expected.add(john);
            assert_eq!(address_book.get_birthday_people(date), Some(expected));

            let mary = Address::new("Ann", "Mary", "1975/09/11", "mary.ann@foobar.com");
            let date = mary.date_of_birth();
            let mut expected = AddressBook::new();
            expected.add(mary);
            assert_eq!(address_book.get_birthday_people(date), Some(expected));

            let date = NaiveDate::parse_from_str("2022/04/03", "%Y/%m/%d").unwrap();
            assert_eq!(address_book.get_birthday_people(date), None);
        }
    }
}
