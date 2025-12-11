use anyhow::Result;

// Slow, we don't need to check so many things
//
// 20 is the first number that could possibly be evenly divisible by all number 1..=20
//
// If something is a multiple of 20 it is also a multiple of 10, 5, 4, 2
// 19 is prime
// If something is a multiple of 18 it is also a multiple of 9, 6, 3, 2
// ...
//
// Could remove all primes from the start, but calculating primes isn't cheap
//
// Order matters too. More numbers are divisible by 2 than 20. Check from highest to lowest.
// All numbers are evenly divisible by 1. No need to check it. 
//
pub fn solve() -> Result<String> {
    let mut i: u64 = 20;
    
    loop {
        if (2..=20).rev().all(|x: u64| i.is_multiple_of(x)) {
            break;
        }

        i += 1;
    }

    Ok(i.to_string())
}

