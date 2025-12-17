use anyhow::Result;
use euler::is_palindrome;

pub fn solve() -> Result<String> {
    let ans: u64 = (1..1_000_000)
        .filter(|&i| is_palindrome(i))
        .filter(|&i| {
            let num_bin = format!("{:b}", i);

            num_bin.chars().eq(num_bin.chars().rev())
        })
        .sum();
    
    Ok(ans.to_string())
}

