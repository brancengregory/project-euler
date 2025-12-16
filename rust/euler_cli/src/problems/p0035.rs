use anyhow::Result;
use std::collections::HashSet;
use euler::{primes_le, num_to_digits, digits_to_num};

pub fn solve() -> Result<String> {
    let candidates: HashSet<u64> = primes_le(1_000_000).into_iter().collect();

    let ans: usize = candidates.iter().filter(|&x| {
        let digits = num_to_digits(*x);
        let len = digits.len();

        let doubled = digits.repeat(2);

        doubled.windows(len)
            .take(len)
            .map(digits_to_num)
            .all(|d| candidates.contains(&d))
    })
        .count();

    Ok(ans.to_string())
}

