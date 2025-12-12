use anyhow::Result;
use euler::factorial;

pub fn solve() -> Result<String> {
    let numerator = factorial(40);
    let denominator = factorial(20) * factorial(20);

    let ans = numerator / denominator;
    
    Ok(ans.to_string())
}

