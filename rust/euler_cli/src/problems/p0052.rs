use anyhow::Result;
use euler::num_to_digits;

pub fn solve() -> Result<String> {
    for x in 1.. {
        let mut x_digits = num_to_digits(x);
        x_digits.dedup();
        x_digits.sort();

        let found = (2..=6).all(|i| {
            let mut ix_digits = num_to_digits(i * x);    
            ix_digits.dedup();
            ix_digits.sort();

            x_digits == ix_digits
        });

        if found {
            return Ok(x.to_string())
        }
    }

    unreachable!()
}

