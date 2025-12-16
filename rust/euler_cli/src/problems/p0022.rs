use anyhow::Result;

pub fn solve() -> Result<String> {
    let mut input: Vec<String> = std::fs::read_to_string("../data/p0022.txt").expect("Couldn't find input")
        .trim().split(",").map(|x| x.replace("\"", "")).collect();

    input.sort();

    let res: u32 = input.iter().enumerate().map(|(i, v)| {
        let char_sum = v.as_bytes().iter().map(|b| { (b - 64) as u32 }).sum::<u32>();
        char_sum * (i + 1) as u32
    }).sum();

    Ok(res.to_string())
}

