use anyhow::Result;
use euler::{triangle_iter, n_divisors};

pub fn solve() -> Result<String> {
    let ans = triangle_iter()
        .find(|&x| n_divisors(x) > 500)
        .expect("Could not find answer");

    Ok(ans.to_string())
}

