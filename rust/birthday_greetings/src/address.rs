pub mod address {
    use chrono::{Datelike, NaiveDate};

    #[derive(Debug, PartialEq, Clone)]
    pub struct Address {
        last_name: String,
        first_name: String,
        date_of_birth: NaiveDate,
        email: String,
    }

    impl Address {
        pub fn new(last_name: &str, first_name: &str, date_of_birth: &str, email: &str) -> Address {
            let last_name = String::from(last_name);
            let first_name = String::from(first_name);
            let date_of_birth =
                NaiveDate::parse_from_str(date_of_birth, "%Y/%m/%d").expect("wrong format");
            let email = String::from(email);
            Address {
                last_name,
                first_name,
                date_of_birth,
                email,
            }
        }

        pub fn is_birthday(&self, target: NaiveDate) -> bool {
            let same_month = self.date_of_birth.month() == target.month();
            let same_day = self.date_of_birth.day() == target.day();
            same_month && same_day
        }

        pub fn date_of_birth(&self) -> NaiveDate {
            self.date_of_birth
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_is_birthday() {
            let address = Address::new("Doe", "John", "1982/10/08", "john.doe@foobar.com");
            let expected_true = NaiveDate::parse_from_str("2022/10/08", "%Y/%m/%d").unwrap();
            assert!(address.is_birthday(expected_true));
            let expected_false = NaiveDate::parse_from_str("1982/10/09", "%Y/%m/%d").unwrap();
            assert!(!address.is_birthday(expected_false));
        }
    }
}
