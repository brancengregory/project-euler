use anyhow::Result;
use euler::{is_pentagonal};

pub fn solve() -> Result<String> {
    let mut pentagonals: Vec<u64> = Vec::new();
    
    for n in 1.. {
        let pk = n * (3 * n - 1) / 2;

        for &pj in pentagonals.iter().rev() {
            let diff = pk - pj;
            let sum = pk + pj;

            if is_pentagonal(diff) && is_pentagonal(sum) {
                return Ok(diff.to_string());
            }
        }

        pentagonals.push(pk);
    } 

    unreachable!()
}

