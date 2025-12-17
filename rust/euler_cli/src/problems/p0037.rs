use anyhow::Result;
use euler::{n_digits, primes};

pub fn solve() -> Result<String> {
	let primes = primes(100_000);

	let ans: Vec<&u64> = primes.iter()
		.filter(|&x| *x > 7)
		.filter(|&x| {
			std::iter::successors(Some(*x), |&n| { Some(n / 10) })
				.take_while(|&n| n > 0)
				.all(|t| primes.contains(&t))
		})
		.filter(|&x| {
			std::iter::successors(Some(*x), |&n| {
				let n_digits = n_digits(n);
				Some(n % 10_u64.pow((n_digits - 1) as u32))
			})
				.take_while(|&n| n > 0)
				.all(|t| primes.contains(&t))
		})
		.collect();

	assert!(ans.len() == 11);

  Ok(ans.into_iter().sum::<u64>().to_string())
}

