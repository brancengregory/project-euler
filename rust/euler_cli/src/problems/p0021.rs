use anyhow::Result;
use euler::is_amicable;

pub fn solve() -> Result<String> {
    let ans: u64 = (1..10_000)
        .filter(|i| is_amicable(*i))
        .sum();

    Ok(ans.to_string())
}

