use anyhow::Result;
use euler::n_digits;

pub fn solve() -> Result<String> {
	let perfect_squares_iter = (0u128..).enumerate().map(|(i, x)| (i, x * x))
		.take_while(|(_, x)| *x < 10_000_000_000_000_000);

	let res: u128 = perfect_squares_iter.filter(|(i, x)| {
		let digits = n_digits((*x).try_into().unwrap());
		let mut modulus = 10;

		for _ in 1..digits {
			let b = x % modulus;
			let a = x / modulus;

			if b > 0 && a + b == *i as u128 && b >= modulus / 10 {
				return true;
			}

			modulus *= 10;
		}

		false
	})
		.map(|(_, x)| x)
		.sum();

	Ok(res.to_string())
}

