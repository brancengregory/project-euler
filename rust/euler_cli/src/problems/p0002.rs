use anyhow::Result;
use euler::fibonacci;

pub fn solve() -> Result<String> {
    let mut fibonaccis = Vec::new();

    let mut i = 0;

    while fibonacci(i) <= 4_000_000 {
        fibonaccis.push(fibonacci(i));
        i += 1;
    }

    let sum: u64 = fibonaccis.iter()
        .filter(|&x| x % 2 == 0)
        .sum();

    Ok(sum.to_string())
}

