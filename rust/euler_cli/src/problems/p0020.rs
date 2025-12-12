use anyhow::Result;
use euler::factorial;

pub fn solve() -> Result<String> {
    let res: u64 = factorial(100).to_radix_le(10).iter().map(|d| *d as u64).sum();
    Ok(res.to_string())
}

