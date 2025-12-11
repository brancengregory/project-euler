use anyhow::Result;
use euler::primes;

pub fn solve() -> Result<String> {
    let ans = primes(10001).last().unwrap().to_owned();
    Ok(ans.to_string())
}

