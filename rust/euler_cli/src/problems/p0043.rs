use anyhow::Result;
use euler::{primes, num_to_digits, digits_to_num, is_zero_pandigital};
use itertools::Itertools;

fn is_substring_divisble(digits: &Vec<u8>) -> bool {
    let primes = primes(7);

    digits.windows(3).skip(1).enumerate()
        .all(|(i, w)| {
            let num = digits_to_num(w);
            num % primes[i] == 0
        })
}

pub fn solve() -> Result<String> {
    let digits: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let zero_pandigitals = digits.into_iter().permutations(10);

    let ans: u64 = zero_pandigitals
        .filter(|i| is_substring_divisble(i))
        .map(|d| digits_to_num(d.as_slice()))
        .sum();

	  Ok(ans.to_string())
}

