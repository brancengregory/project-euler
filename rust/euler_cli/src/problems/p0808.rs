use anyhow::Result;
use euler::{reverse_int, primes};


pub fn solve() -> Result<String> {
	let squared_primes = primes(1000).into_iter()
		.map(|p| p * p);

	let irreversible_squared_primes = squared_primes.filter(|&sp| {
		reverse_int(sp) != sp
	});


	Ok("".to_string())
}

