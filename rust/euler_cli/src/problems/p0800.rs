use anyhow::Result;
use euler::primes_le;
use rayon::prelude::*;

pub fn solve() -> Result<String> {
	let limit = 800800;
	let limit_float = limit as f64;
	let limit_ln = limit_float * limit_float.ln();

	let prime_limit = 15_704_561;

	let primes = primes_le(prime_limit);

	let ans: usize = primes.par_iter().enumerate().map(|(i, &p)| {
		let p_float = p as f64;
		let p_ln = p_float.ln();

		let candidates = &primes[i + 1..];

		// This lets us skip q values once we find one that exceeds the limit
		// If our problem wasn't monotonic this would fail.
		// I.e. if q+1 could ever meet a condition that q fails we couldn't
		// use this partition method.
		// But p^q+1 > p^q for all q, so we are good.
		candidates.partition_point(|&q| {
			let q_float = q as f64;
			let q_ln = q_float.ln();

			let val = (q_float * p_ln) + (p_float * q_ln);
			val <= limit_ln
		})
	})
		.sum();

	println!("{:?}", ans);

	Ok("".to_string())
}

