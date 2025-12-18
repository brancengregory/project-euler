use anyhow::Result;
use euler::primes;

pub fn solve() -> Result<String> {
    let primes = primes(100_000);
    
    let odd_composites = std::iter::successors(Some(9), |x| Some(x + 2))
        .filter(|x| !primes.contains(x));

    let doubled_powers_of_2 = (1..).map(|x: u64| 2 * x.pow(2));

    for x in odd_composites {
        for y in doubled_powers_of_2.clone() {
            if y >= x {
                return Ok(x.to_string())
            }

            let diff = x - y;

            if primes.contains(&diff) {
                break
            } else {
                continue
            }
        }
    }

    unreachable!()
}

