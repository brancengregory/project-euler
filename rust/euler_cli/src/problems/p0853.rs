use anyhow::Result;

fn pisano(m: u64) -> u64 {
	let mut prev = 0;
	let mut current = 1;
	let mut res = 0;

	for i in 0..(m * m) {
		let temp = current;
		current = (prev + current) % m;
		prev = temp;

		if prev == 0 && current == 1 {
			res = i + 1;
			break;
		}
	}

	res
}

pub fn solve() -> Result<String> {
	let limit: u64 = 1_000_000_000;
	let ans = (1..limit)
		.filter(|&n| pisano(n) == 120)
		.count();

	println!("{:?}", ans);

	Ok("".to_string())
}

