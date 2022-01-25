struct PrimeFactors {}

impl PrimeFactors {
    fn generate(mut num: u32) -> Vec<u32> {
        let mut primes = vec![];
        let mut candidate = 2;
        while num > 1 {
            while num % candidate == 0 {
                primes.push(candidate);
                num /= candidate;
            }
            candidate += 1;
        }
        primes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate(num: u32) -> Vec<u32> {
        PrimeFactors::generate(num)
    }

    #[test]
    fn test_one() {
        assert_eq!(vec![] as Vec<u32>, generate(1));
    }

    #[test]
    fn test_two() {
        assert_eq!(vec![2], generate(2));
    }

    #[test]
    fn test_three() {
        assert_eq!(vec![3], generate(3));
    }

    #[test]
    fn test_four() {
        assert_eq!(vec![2, 2], generate(4));
    }

    #[test]
    fn test_five() {
        assert_eq!(vec![5], generate(5));
    }

    #[test]
    fn test_six() {
        assert_eq!(vec![2, 3], generate(6));
    }

    #[test]
    fn test_eight() {
        assert_eq!(vec![2, 2, 2], generate(8));
    }

    #[test]
    fn test_nine() {
        assert_eq!(vec![3, 3], generate(9));
    }
}
