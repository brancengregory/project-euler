use anyhow::Result;
use euler::factorial;
use num::bigint::ToBigUint;

pub fn solve() -> Result<String> {
    let mut count = 0;

    (1..=100).for_each(|n| {
        (1..n).rev().for_each(|r| {
            let num = factorial(n);
            let denom = factorial(r) * factorial(n - r);
            let val = num / denom;
            if val > 1_000_000u64.to_biguint().expect("Couldn't turn u64 into biguint") {
                count += 1; 
            }
        })
    });

    Ok(count.to_string())
}

