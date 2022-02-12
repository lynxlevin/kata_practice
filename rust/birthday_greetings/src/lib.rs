mod address;
mod address_book;
use crate::address::address::Address;
use crate::address_book::address_book::AddressBook;
use quick_csv::Csv;

fn parse_csv(file_path: &str) -> AddressBook {
    let csv = Csv::from_file("addresses/address.csv").unwrap();
    let csv = csv.has_header(true);
    let mut address_book = AddressBook::new();
    for row in csv.into_iter() {
        let row = row.unwrap();
        if let Ok(mut columns) = row.columns() {
            let address = Address::new(
                columns.next().unwrap(),
                columns.next().unwrap().trim(),
                columns.next().unwrap().trim(),
                columns.next().unwrap().trim(),
            );
            address_book.add(address);
        }
    }
    address_book
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
    fn test_parse_csv() {
        let result = parse_csv("addresses/address.csv");

        let address_book = create_address_book();
        assert_eq!(result, address_book);
    }
}
