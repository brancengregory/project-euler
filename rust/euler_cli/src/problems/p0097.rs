use anyhow::Result;
use num::BigUint;

pub fn solve() -> Result<String> {
    let modulus: u64 = 10_000_000_000;
    let big_modulus = BigUint::from(10_000_000_000u64);
    // a * b + c
    let a = 28433 % modulus;
    let c = 1 % modulus;

    let base = BigUint::from(2u64);
    let exp = BigUint::from(7830457u64);
    let b = base.modpow(&exp, &big_modulus);

    let res = (a * b + c) % modulus;

    Ok(res.to_string())
}

