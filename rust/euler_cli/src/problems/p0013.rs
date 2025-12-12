use anyhow::Result;

pub fn solve() -> Result<String> {
    let input: Vec<Vec<u32>> = std::fs::read_to_string("../data/p0013.txt").expect("Could not find input file")
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).expect("Couldn't convert character to digit"))
                .collect()
        })
        .collect();

    let mut sum_digits = Vec::new();
    let mut carry = 0;

    for col in (0..50).rev() {
        let col_sum: u32 = input.iter()
            .map(|row| row[col])
            .sum();

        let current_val = col_sum + carry;

        sum_digits.push(current_val % 10);

        carry = current_val / 10;
    }

    while carry > 0 {
        sum_digits.push(carry % 10);
        carry /= 10;
    }

    sum_digits.reverse();

    let ans: String = sum_digits.iter()
        .take(10)
        .map(|d| char::from_digit(*d, 10).expect("Could not parse digit to character"))
        .collect();

    Ok(ans)
}

