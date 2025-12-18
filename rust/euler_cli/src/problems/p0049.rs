use anyhow::Result;
use euler::{are_permutations, primes_le};

pub fn solve() -> Result<String> {
    let primes: Vec<u64> = primes_le(9999).into_iter().filter(|&x| x >= 1000).collect();

    for (i, &p1) in primes.iter().enumerate() {
        for &p2 in primes.iter().skip(i + 1) {
            let diff = p2 - p1;
            let p3 = p2 + diff;

            if p3 >= 10_000 { break; }

            if p1 != 1487 && primes.contains(&p3) && are_permutations(p1, p2) && are_permutations(p1, p3) {
                return Ok(format!("{}{}{}", p1, p2, p3))
            }
        }
    }
    
    unreachable!()
}

