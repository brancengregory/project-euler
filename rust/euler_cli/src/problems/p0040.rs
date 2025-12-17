use anyhow::Result;
use euler::{primes_le, is_n_pandigital};

pub fn solve() -> Result<String> {
	let primes = primes_le(10_000_000);
	let ans = primes.iter().rev()
		.filter(|&p| is_n_pandigital(*p, 7))
		.max()
		.expect("Couldn't find max pandigital prime");

	Ok(ans.to_string())
}

