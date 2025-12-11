use anyhow::Result;
use euler::triangle_numbers;

pub fn solve() -> Result<String> {
    let tri_nums = triangle_numbers(500);
    println!("{:?}", tri_nums);
    Ok("".to_string())
}

