use anyhow::Result;
use std::ops::Rem;
use num::{BigUint, Zero};

pub fn solve() -> Result<String> {
    let sum: BigUint = (1..=1000u32).fold(BigUint::zero(), |acc, x: u32| {
        let base = BigUint::from(x);
        acc + base.pow(x)
    });

    let res: BigUint = sum.rem(BigUint::from(10_000_000_000u64));

    Ok(res.to_string())
}

