use anyhow::Result;
use euler::primes_le;

pub fn solve() -> Result<String> {
    let limit = 1_000_000;
    let primes = primes_le(limit);

    let mut consecutive_sums: Vec<u64> = Vec::new();
    let mut current_sum = 0;

    for &p in primes.iter() {
        current_sum += p;
        
        if current_sum >= (limit * 2) {
            break;
        }

        consecutive_sums.push(current_sum);
    }

    for len in (0..consecutive_sums.len()).rev() {
        for i in 0..(consecutive_sums.len() - len) {
            let sum = consecutive_sums[i + len] - consecutive_sums[i];

            if sum >= limit {
                break;
            }

            if primes.binary_search(&sum).is_ok() {
                return Ok(sum.to_string())
            }
        }
    }


    unreachable!()
}

