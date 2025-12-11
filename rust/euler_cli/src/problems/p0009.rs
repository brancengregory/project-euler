use anyhow::Result;

//
// Knowns:
// a, b, c > 0
// a < b < c
// a^2 + b^2 = c^2
// a + b + c = 1_000
//
// Therefore:
// sqrt(a^2 + b^2) = 1_000 - a - b
//
// We just need to find a and b and can derive c
// 
// If c = 997, then b = 2 and a = 1 is the smallest possible values for b and a
// What are the max values of a and b possible?
// c = b + 1
// b = a + 1
// c = 1_000 - a - b
// b + 1 = 1_000 - a - a - 1
// b = 998 - 2a
// b = 998 - 2(b - 1)
// b = 998 - 2b + 2
// 3b = 1000
// b = 1000/3
//
// That is allowing a, b, and c to be continuous though, not constrained to natural numbers
//
// Instead look at:
// a^2 + b^2 = (1_000 - a - b)^2
//
// Simplified to:
// b = (1000 * (500 - a)) / (1000 - a)
// 
// Now we only need to iterate over a and can derive b and c
//
// We can even find an upper bound for a.
// 
// Since a < b:
// a < (1000 * (500 - a)) / (1000 - a)
// 
// Simplify to:
// a^2 - 2000a + 500_000 > 0
//
// Calculate roots of a^2 - 2000a + 500_000 = 0 using quadratic formula:
// ~1707.107, ~292.8932
//
// 1707 is out of bounds, so we take:
// a < 292.8932
//
// We can iterate a from 1 to 292 and we are guaranteed to have it covered!
// 
pub fn solve() -> Result<String> {
    let [mut a, mut b, mut c] = [0u64; 3];

    for i in 1..=292 {
        a = i;
        b = (1000 * (500 - a)) / (1000 - a);
        c = 1000 - a - b;

        if a.pow(2) + b.pow(2) == c.pow(2) {
            println!("a: {}, b: {}, c: {}", a, b, c);
            break;
        }
    }

    let ans = a * b * c;

    Ok(ans.to_string())
}

