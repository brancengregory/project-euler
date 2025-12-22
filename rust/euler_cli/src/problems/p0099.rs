use anyhow::Result;

pub fn solve() -> Result<String> {
	let input: Vec<_> = std::fs::read_to_string("../data/p0099.txt")
		.expect("Couldn't find input file")
		.lines()
		.map(|l| {
        l.split(",")
            .map(|x| {
              x.parse::<u64>().expect("Couldn't parse number")
            })
            .collect::<Vec<_>>()
    })
		.collect();

  let ans: (usize, f64) = input.iter().enumerate()
    .map(|(i, x)| {
        let a = x[0] as f64;
        let b = x[1] as f64;
        (i, b * a.ln())
    })
    .fold((0, f64::NEG_INFINITY), |acc: (usize, f64), x| {
        if x.1 > acc.1 {
            x
        } else {
            acc
        }
    });

	Ok((ans.0 + 1).to_string())
}

