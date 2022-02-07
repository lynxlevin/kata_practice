use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
extern crate quick_csv;

fn parse_csv(data: &str) -> quick_csv::Csv<&[u8]> {
    let mut csv = quick_csv::Csv::from_string(data);

    csv
}

fn read_file_to_string(file_path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv() {
        let result = match read_file_to_string("addresses/address.csv") {
            Ok(string) => string,
            Err(err) => err.to_string(),
        };
        let mut csv = parse_csv(&result);

        let row = csv.next().unwrap().unwrap();

        if let Ok((col1, col2, col3, col4)) = row.decode::<(String, String, String, String)>() {
            println!(
                "col1: {}, col2: {}, col3: {}, col4: {}",
                col1, col2, col3, col4
            );
        }
        println!("hellohello");
        assert_eq!("hi", "");
    }

    #[test]
    fn test_read_file_to_string_ok() {
        let result = match read_file_to_string("addresses/address.csv") {
            Ok(string) => string,
            Err(err) => err.to_string(),
        };
        assert_eq!(
            result,
            "last_name, first_name, date_of_birth, email
Doe, John, 1982/10/08, john.doe@foobar.com
Ann, Mary, 1975/09/11, mary.ann@foobar.com"
        );
    }

    #[test]
    fn test_read_file_to_string_err() {
        let result = match read_file_to_string("addresses/addre.csv") {
            Ok(string) => string,
            Err(err) => err.to_string(),
        };
        assert_eq!(result, "No such file or directory (os error 2)");
    }
}
