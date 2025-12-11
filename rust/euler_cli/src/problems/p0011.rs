use anyhow::Result;

// We are looking for 4 (n) adjacent numbers (including diagonally adjacent)
// that have maximum product.
//
// Left and right are equivalent.
// Up and down are equivalent.
// Diagonal can be \ or /
// 
// This can be done in 1 sweep, checking each i for only some things.
// If i is (0, 0) we would do -> v and \
// Middle-ish i's (3+, i.e. n-1) we check v -> \ and /
// We don't need to ever check <- or ^
// For i to the far top right (len() - 1, 0) we just check v and / 
// Once we get to the bottom we stop checking v \ and /, and only check ->
// This means the last i to check is in position (len() - n, len() - 1)
//
pub fn solve() -> Result<String> {
    let input: Vec<Vec<u64>> = std::fs::read_to_string("../data/p0011.txt").expect("Couldn't find input file")
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| {
                    x.parse::<u64>().expect("Couldn't parse string to u64")
                })
                .collect::<Vec<u64>>()
        })
        .collect();
    
    let n: usize = 4;
    
    let width = input[0].len();
    let height = input.len();

    let mut max_prod = 0;

    for j in 0..height {
        for i in 0..width {
            let current = (i, j);
            let current_val = input[i][j];

            // Right
            if current.0 <= (width - n) {
                max_prod = (1..=3)
                    .map(|x| input[current.0 + x][current.1])
                    .fold(current_val, |acc, x| acc * x)
                    .max(max_prod)
            }

            // Down
            if current.1 <= (height - n) {
                max_prod = (1..=3)
                    .map(|x| input[current.0][current.1 + x])
                    .fold(current_val, |acc, x| acc * x)
                    .max(max_prod)
            }

            // Forward Slash
            if current.0 <= (width - n) && current.1 <= (height - n) {
                max_prod = (1..=3)
                    .map(|x| input[current.0 + x][current.1 + x])
                    .fold(current_val, |acc, x| acc * x)
                    .max(max_prod)
            }

            // Backward Slash
            if current.0 >= (n - 1) && current.1 <= (height - n) {
                max_prod = (1..=3)
                    .map(|x| input[current.0 - x][current.1 + x])
                    .fold(current_val, |acc, x| acc * x)
                    .max(max_prod)
            }
        }
    }

    Ok(max_prod.to_string())
}

