use anyhow::Result;
use euler::{is_hexagonal, is_pentagonal, is_triangular};

pub fn solve() -> Result<String> {
    for n in 40756.. {
        if is_hexagonal(n) && is_pentagonal(n) && is_triangular(n) {
            return Ok(n.to_string())
        }
    } 

    unreachable!()
}

