use anyhow::Result;
use euler::is_palindrome;

pub fn solve() -> Result<String> {
    let mut largest_num: u64 = 0;
    
    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            let prod = i * j;
            if is_palindrome(prod) {
                largest_num = largest_num.max(prod)
            }
        }
    }

    Ok(largest_num.to_string())
}

