use anyhow::Result;
use euler::{num_to_digits, digits_to_num};

fn sort_nonzero_digits(n: u64) -> Vec<u8> {
	let mut digits = num_to_digits(n);
	digits.retain(|d| *d != 0);
	digits.sort();
	digits
}

pub fn solve() -> Result<String> {
	let modulus = 1_123_455_689;
	let test: u64 = (1..1_000_000_000_000_000_000).map(|d| {
		let sorted_digits = sort_nonzero_digits(d);
		digits_to_num(&sorted_digits) % modulus
	})
		.sum();

	println!("{:?}", test);

  Ok("".to_string())
}

