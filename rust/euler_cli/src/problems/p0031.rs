use anyhow::Result;

pub fn solve() -> Result<String> {
    let target: usize = 200;
    let coins: [usize; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    let mut ways: Vec<u32> = vec![0; target + 1];
    ways[0] = 1;

    coins.iter().for_each(|&c| {
        (c..ways.len()).for_each(|i| {
            ways[i] += ways[i - c]
        })
    });
    
    Ok(ways[target].to_string())
}

