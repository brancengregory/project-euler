use anyhow::Result;

// a^2 + b^2 = c^2
// where a + b + c = P
// and a, b, and c are integers
// and a < b < c
//
// if P = 120
// a, b, c equals either
// 20, 48, 52
// 24, 45, 51
// or 30, 40, 50
//
// c = sqrt(a^2 + b^2)
// c = P - a - b
// P - a - b = sqrt(a^2 + b^2)
// (P - a - b)^2 = a^2 + b^2
// (P - a - b)(P - a - b) = a^2 + b^2
// P^2 -aP -bP -aP +a^2 +ab -bP +ab +b^2
// P^2 -2aP -2bP +a^2 + b^2 +2ab = a^2 + b^2
// P^2 -2aP -2bP +2ab = 0
// P^2 - 2aP = 2bP -2ab
// P^2 - 2aP = 2b(P - a)
// b = P^2 - 2aP / 2(P-a)
//
pub fn solve() -> Result<String> {
	let perimeters: std::ops::RangeInclusive<i32> = 1..=1000;

	let ans: Vec<(i32, usize)> = perimeters.filter_map(|p| {
		let mut solutions = Vec::new();
		for a in 1..=(p / 3) {
			let denom = 2 * (p - a);
			if denom == 0 { continue };

			let numerator = p.pow(2) - (2 * a * p);

			if numerator % denom != 0 { continue };

			let b =  numerator / denom;
			let c = p - a - b;

			if b <= 0 || c <= 0 { continue };

			solutions.push((a, b, c));
		}
		if solutions.len() == 0 {
			None
		} else {
			Some((p, solutions.len()))
		}
	})
		.collect();

	let res = ans.iter()
		.max_by(|&a, &b| {
			a.1.cmp(&b.1)
		})
		.expect("Couldn't get max perimeter")
		.0;

	Ok(res.to_string())
}
