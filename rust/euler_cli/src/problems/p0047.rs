use anyhow::Result;
use euler::{distinct_prime_factors};

pub fn solve() -> Result<String> {
    let mut n = 0;
    let mut i = 1;
    let mut nums = Vec::new();

    loop {
        if distinct_prime_factors(i).len() == 4 {
            n += 1;
            nums.push(i);
        } else {
            n = 0;
            nums.clear();
        }

        if n == 4 {
            break;
        }

        i += 1;
    }

    assert!(nums.len() == 4);

    Ok(nums[0].to_string())
}

