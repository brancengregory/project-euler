use anyhow::Result;
use num::BigUint;

pub fn solve() -> Result<String> {
	let res = BigUint::from(2u8).pow(1000);

	let ans: u64 = res
		.to_radix_le(10)
		.iter()
		.map(|d| *d as u64)
		.sum();

  Ok(ans.to_string())
}

