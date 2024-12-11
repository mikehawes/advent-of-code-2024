pub fn count_digits(stone: usize) -> usize {
    let log = (stone as f32).log10();
    log.floor() as usize + 1
}

pub fn split_even_digits(stone: usize, digits: usize) -> Vec<usize> {
    let split = digits / 2;
    let mut higher = 0;
    let mut lower = 0;
    let mut acc = stone;
    let mut mult = 1;
    for _ in 0..split {
        let digit = acc % 10;
        lower += digit * mult;
        mult *= 10;
        acc /= 10;
    }
    mult = 1;
    for _ in split..digits {
        let digit = acc % 10;
        higher += digit * mult;
        mult *= 10;
        acc /= 10;
    }
    vec![higher, lower]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_count_digits() {
        assert_eq!(count_digits(1234), 4)
    }

    #[test]
    fn can_count_digits_ending_in_zero() {
        assert_eq!(count_digits(1234567890), 10)
    }

    #[test]
    fn can_count_one_digit() {
        assert_eq!(count_digits(1), 1)
    }

    #[test]
    fn can_count_digits_of_10() {
        assert_eq!(count_digits(10), 2)
    }

    #[test]
    fn can_split_digits_of_123456() {
        assert_eq!(split_even_digits(123456, 6), vec![123, 456])
    }

    #[test]
    fn can_split_digits_of_253000() {
        assert_eq!(split_even_digits(253000, 6), vec![253, 0])
    }
}
