/// Sums the digits in a non-negative integer.
pub fn sum_digits(n: u64) -> u64 {
    let mut r = 0u64;
    let mut n = n;

    while n > 0 {
        r += n % 10;
        n /= 10;
    }

    r
}

#[cfg(test)]
mod tests {
    use super::sum_digits;

    #[test]
    fn test_sum_digits() {
        let inputs_and_expected: Vec<(u64, u64)> = vec![
            (0, 0),
            (1, 1),
            (10, 1),
            (15, 6),
            (247, 13),
            (1000, 1),
            (123456, 21),
        ];

        for (input, expected) in inputs_and_expected {
            let produced = sum_digits(input);
            assert_eq!(expected, produced);
        }
    }
}
