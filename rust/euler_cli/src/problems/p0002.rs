use anyhow::Result;

fn fibonacci(i: u32) -> u32 {
    match i {
        0 => return 0,
        1 => return 1,
        _ => fibonacci(i - 1) + fibonacci(i - 2)
    }
}

pub fn solve() -> Result<String> {
    let mut fibonaccis = Vec::new();

    let mut i = 0;

    while fibonacci(i) <= 4_000_000 {
        fibonaccis.push(fibonacci(i));
        i += 1;
    }

    let sum: u32 = fibonaccis.iter()
        .sum();

    Ok(sum.to_string())
}

