use anyhow::Result;
use euler::{reverse_int, is_palindrome};

pub fn solve() -> Result<String> {
	let ans = (1..10_000).filter(|&x| {
		std::iter::successors(Some(x), |&curr| {
			let next = curr + reverse_int(curr);
			Some(next)
		})
			.skip(1)
			.take(50)
			.all(|sum| !is_palindrome(sum))
	})
		.count();

	Ok(ans.to_string())
}

