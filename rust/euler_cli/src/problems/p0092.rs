use anyhow::Result;
use std::collections::HashMap;
use euler::num_to_digits;

fn sum_squared_digits(n: u64) -> u64 {
	let digits = num_to_digits(n);
	digits.iter().map(|i| (i * i) as u64).sum()
}

fn terminating_number(n: u64, hm: &mut HashMap<u64, u64>) -> u64 {
	if let Some(x) = hm.get(&n) {
		return *x
	}

	let mut current_num = n;
	let mut unknowns = Vec::new();
	let terminating_num;

	loop {
		unknowns.push(current_num);

		if current_num == 1 || current_num == 89 {
			terminating_num = current_num;
			break;
		}

		let next_num = sum_squared_digits(current_num);

		if let Some(&val) = hm.get(&next_num) {
			terminating_num = val;
			break;
		}

		current_num = next_num;
	}

	for u in unknowns {
		hm.insert(u, terminating_num);
	}

	terminating_num
}

pub fn solve() -> Result<String> {
	let limit = 10_000_000;

	let mut hm: HashMap<u64, u64> = HashMap::new();
	hm.insert(1, 1);
	hm.insert(89, 89);
	hm.insert(44, 1);
	hm.insert(85, 89);

	let ans = (1..limit).map(|x| {
		terminating_number(x, &mut hm)
	})
		.filter(|&x| x == 89)
		.count();

	Ok(ans.to_string())
}

