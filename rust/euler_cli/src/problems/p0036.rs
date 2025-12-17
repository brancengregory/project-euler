use anyhow::Result;
use euler::is_palindrome;

pub fn solve() -> Result<String> {
    let ans: u64 = (1..1_000_000).filter_map(|i| {
        if is_palindrome(i) {
            let num_bin = format!("{:b}", i);
            let rev_bin: String = num_bin.chars().rev().collect();

            if rev_bin == num_bin {
                Some(i)
            } else {
                None
            }
        } else {
            None
        }
    }).sum();
    
    Ok(ans.to_string())
}

