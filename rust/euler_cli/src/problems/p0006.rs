use anyhow::Result;
use euler::{sum_squares, squared_sum};

pub fn solve() -> Result<String> {
    let ans = squared_sum(100) - sum_squares(100);
    Ok(ans.to_string())
}

