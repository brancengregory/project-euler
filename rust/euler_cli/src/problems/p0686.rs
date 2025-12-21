use anyhow::Result;

// p(L, n) is the nth smallest value of j such
// that the base 10 rep of 2^j begins with
// the digits of L
//
// ex. p(12, 1) = 7
// p(12, 2) = 80
// p(123, 45) = 12710
//
// 2^j = m * 10^k
//
// Where 1 <= m < 10.
// If 1.23 <= m < 1.24 then the number counts
//
// log_10(2^j) = log_10(m * 10^k)
// j log_10(2) = log_10(m) + log_10(10^k)
// j log_10(2) = log_10(m) + k log_10(10)
// j log_10(2) = log_10(m) + k
// log_10(m) = j log_10(2) - k
// m = 10^(j log_10(2) - k)
//
// 0 <= log_10(m) < 1
// So log_10(m) is the fractional part of j log_10(2)
// and k is the integer portion
//
// j log_10(2) is about 12345.0899
// k tells you how many digits the number has minus 1
// 0.0899 determines the leading digits
//
pub fn solve() -> Result<String> {
	let lower = 1.23_f64.log10();
	let upper = 1.24_f64.log10();

	let mut j = 1;
	let mut val: f64;
	let mut nth = 0;
	let res: u64;
	let log2 = 2.0_f64.log10();

	loop {
		val = j as f64 * log2;
		let fract = val.fract();

		if fract >= lower && fract < upper {
			nth += 1;

			if nth == 678910 {
				res = j;
				break;
			}
		};

		j += 1;
	}

	Ok(res.to_string())
}

