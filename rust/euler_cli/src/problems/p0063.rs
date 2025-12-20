use anyhow::Result;

// For a number to have a certain number of digits it must be in the range
// exponent = 5
// 10^4 <= x < 10^5
//
// ie. for exponent n, 10^(n-1) <= x < 10^n
//
// We are looking for x = b^n so:
// 10^(n-1) <= b^n < 10^n
//
// The right-hand side says b < 10
// On the left-hand side, If b were 9 (max val)
// then 10^(n-1) <= 9^n
//
// ln 10^(n-1) <= ln 9^n
// (n-1) * ln 10 <= n * ln 9
// n ln 10 - ln 10 <= n * ln 9
// n ln 10 - n ln 9 <= ln 10
// n (ln 10 - ln 9) <= ln 10
// n <= ln 10 / (ln 10 - ln 9)
// n <= 21.85435
//
// So we check b 1-9
// and n 1-21
//
pub fn solve() -> Result<String> {
	let ans = (1..=9_u32).flat_map(|b| {
		(1..=21_u32).filter(move |&n| {
			((n - 1) as f64) * 10f64.ln() <= (n as f64) * (b as f64).ln()
		})
	})
		.count();

	Ok(ans.to_string())
}

