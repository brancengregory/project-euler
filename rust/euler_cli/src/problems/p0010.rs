use anyhow::Result;
use euler::primes_le;

pub fn solve() -> Result<String> {
    let ans: u64 = primes_le(2_000_000).iter().sum();

    Ok(ans.to_string())
}

