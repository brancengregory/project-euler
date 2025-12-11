use anyhow::Result;

pub fn solve() -> Result<String> {
    let input: String = std::fs::read_to_string("../data/p0008.txt").expect("Could not find input file")
        .split_whitespace()
        .collect();

    let digits: Vec<u64> = input.chars()
        .map(|x| x.to_digit(10).expect("Couldn't convert char to digit") as u64)
        .collect();

    let ans = digits.windows(13).map(|x| x.iter().fold(1, |acc, x| acc * x)).max().expect("Couldn't find max");

    Ok(ans.to_string())
}

