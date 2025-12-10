use anyhow::Result;

// In the future: use euler::primes;

pub fn solve() -> Result<String> {
    // Logic for Problem 1
    let sum: u32 = (0..1000)
        .filter(|&x| x % 3 == 0 || x % 5 == 0)
        .sum();

    Ok(sum.to_string())
}
