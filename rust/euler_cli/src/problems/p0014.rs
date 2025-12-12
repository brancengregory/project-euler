use anyhow::Result;

pub fn solve() -> Result<String> {
	let mut max_seq = (2, 0);

	for i in 2..1_000_000 {
		let mut current_val: u32 = i;
		let mut seq_len = 1;

		while current_val != 1 {
			if current_val % 2 == 0 {
				current_val /= 2;
			} else {
				current_val = 3 * current_val + 1;
			}

			seq_len += 1;
		}

		if seq_len > max_seq.1 {
			max_seq = (i, seq_len);
		}
	}

  Ok(max_seq.0.to_string())
}

