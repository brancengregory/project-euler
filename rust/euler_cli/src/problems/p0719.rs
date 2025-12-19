use anyhow::Result;
use euler::{digits_to_num, num_to_digits};
use rayon::prelude::*;

fn is_s_number(n: u64) -> bool {
	let root = (n as f64).sqrt() as u64;
	if root * root != n { return false; }

	if n % 9 != root % 9 { return false; }

	let digits = num_to_digits(n);
	let n_gaps = digits.len() - 1;
	let max_mask = (1 << n_gaps) - 1;

	for mask in 1..=max_mask {
		let mut start: usize = 0;
		let mut sum = 0;

		for i in 0..n_gaps {
			if (mask >> i) &1 == 1 {
				let end = i as usize + 1;
				sum += digits_to_num(&digits[start..end]);
				start = end;
				if sum > root { break; }
			}
		}

		sum += digits_to_num(&digits[start..]);

		if sum == root as u64 {
			return true;
		}
	}

	false
}

pub fn solve() -> Result<String> {
	let limit: u64 = 10_u64.pow(12);
	// Max root is limit.sqrt()
	let max_root = 1_000_000;

	let ans: u64 = (1..=max_root)
		.into_par_iter()
		.map(|r| r * r)
		.filter(|&sq| is_s_number(sq))
		.sum();

	Ok(ans.to_string())
}
