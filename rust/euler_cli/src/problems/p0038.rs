use anyhow::Result;
use itertools::Itertools;
use euler::{is_pandigital, digits_to_num};

// 2
// 2*1 = 2 2*2 = 4 2*3 = 6, ...
// p1 = 2, p2 = 4, p3 = 6, ...
// p1 * 10^0 + p2 * 10^3 + p3 * 10^6 + ...
// 123456789 <= (n * 1 * 10^0) + (n * 2 * 10^3) + (n * 3 * 10^6) + ... <= 987654321
//
// n must be 4 or less digits.
// Likely to start with a 9

pub fn solve() -> Result<String> {
	let largest_pandigital = 987654321;
	let smallest_pandigital = 123456789;

	let pandigitals: Vec<_> = (1..=9).permutations(9)
		.map(|x| {
			digits_to_num(x.as_slice())
		})
		.collect();

	let ans = (2..=9999).rev().filter_map(|n| {
		let mut total = String::new();
		let mut i = 1;
		loop {
			total.push_str(&(n * i).to_string());

			if total.len() >= 9 {
				break
			}

			i += 1;
		}

		let val = total.parse::<u64>().expect("Couldn't turn string into number");

		if is_pandigital(val) {
			Some(val)
		} else {
			None
		}
	})
	.max()
	.expect("Couldn't find max pandigital");

  Ok(ans.to_string())
}

