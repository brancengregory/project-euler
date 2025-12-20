use anyhow::Result;
use euler::fibonacci;
use num::BigUint;

pub fn solve() -> Result<String> {
	let modulus: BigUint = BigUint::from(1_000_000_007u64);
	let fibs = (2..=90).map(|i| fibonacci(i));

	let big_sum: BigUint = fibs.map(|l| {
		let q = l / 9;
		let r = l % 9;

		let q_big = BigUint::from(q);
		let base = BigUint::from(10u32);

		let e = base.modpow(&q_big, &modulus);
		let t1 = (BigUint::from(6u32) * (&e - BigUint::from(1u32) + &modulus)) % &modulus;
		let t2 = (BigUint::from(9u32) * (q_big % &modulus)) % &modulus;
		let c = (r * (r + 3)) / 2;
		let t3 = (e * c) % &modulus;

		let p1 = (t1 + &modulus - t2) % &modulus;
		let p2 = (t3 + &modulus - r) % &modulus;

		(p1 + p2) % &modulus
	})
		.sum();

	let ans = big_sum % &modulus;

	Ok(ans.to_string())
}

