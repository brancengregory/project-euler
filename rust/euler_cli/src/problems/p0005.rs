use anyhow::Result;

pub fn solve() -> Result<String> {
    let mut i: u64 = 1;
    
    loop {
        if (1..=20).all(|x: u64| i.is_multiple_of(x)) {
            break;
        }
    
        i += 1;
    }

    Ok(i.to_string())
}

