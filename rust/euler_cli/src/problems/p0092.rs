use anyhow::Result;
use euler::num_to_digits;

fn sum_squared_digits(n: u64) -> u64 {
	let digits = num_to_digits(n);
	digits.iter().map(|i| (i * i) as u64).sum()
}

pub fn solve() -> Result<String> {
	let limit = 10_000_000;

	// Take advantage of the fact that every sequence will get to this range
	// within 1 step since it is based only on digits
	let mut cache = [0u64; 568];

	for i in 1..568 {
		let mut curr = i as u64;
		while curr != 1 && curr != 89 {
			curr = sum_squared_digits(curr);
		}
		cache[i] = curr;
	}

	let ans = (1..limit)
		.filter(|&x| {
			let next = sum_squared_digits(x);
			cache[next as usize] == 89
		})
		.count();

	Ok(ans.to_string())
}

