use anyhow::Result;
use euler::primes_le;

pub fn solve() -> Result<String> {
    let num: u64 = 600_851_475_143;
    let primes = primes_le(num.isqrt());
    let mut ans: u64 = 0;

    for n in primes.iter().rev() {
        if num % n == 0 {
            ans = *n;
            break;
        }
    }
    
    Ok(ans.to_string())
}

