use anyhow::Result;

pub fn solve() -> Result<String> {
    let input: Vec<Vec<u32>> = std::fs::read_to_string("../data/p0018.txt").expect("Couldn't find input")
        .lines()
        .map(|x| x.split_whitespace().map(|n| n.parse::<u32>().expect("Couldn't parse to u32")).collect())
        .rev()
        .collect();

    let mut row_iter = input.iter();
    let init = row_iter.next().expect("No last row found");
    let mut winners: Vec<u32> = init.windows(2).map(|a| a[0].max(a[1])).collect();

    for row in row_iter {
        assert!(winners.len() == row.len());

        for (i, n) in winners.iter_mut().enumerate() {
            *n += row[i];
        }

        if winners.len() == 1 {
            break;
        }
        
        winners = winners.windows(2).map(|a| a[0].max(a[1])).collect();
    }

    Ok(winners[0].to_string())
}

