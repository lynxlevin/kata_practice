fn crack(text: &str) -> String {
    let mut result = String::from("");
    for c in text.chars() {
        result.push(match c {
            '!' => 'a',
            ')' => 'b',
            '"' => 'c',
            '(' => 'd',
            '£' => 'e',
            '*' => 'f',
            '%' => 'g',
            '&' => 'h',
            '>' => 'i',
            '<' => 'j',
            '@' => 'k',
            'a' => 'l',
            'b' => 'm',
            'c' => 'n',
            'd' => 'o',
            'e' => 'p',
            'f' => 'q',
            'g' => 'r',
            'h' => 's',
            'i' => 't',
            'j' => 'u',
            'k' => 'v',
            'l' => 'w',
            'm' => 'x',
            'n' => 'y',
            'o' => 'z',
            _ => c,
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cracking() {
        let result = crack("!)\"(£*%&><@abcdefghijklmno");
        assert_eq!(result, "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn test_crack() {
        let result = crack("T&>h >h &!g(.");
        assert_eq!(result, "This is hard.");
    }
}
