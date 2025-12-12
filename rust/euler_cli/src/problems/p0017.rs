use anyhow::Result;
use euler::num_to_words;

// If num < 10 we use lookup on it's only digit.
// For 11-19 just use a lookup still.
// For multiples of 10 > 10, < 100 just use a lookup.
// For numbers > 20 not multiple of 10 we combine the multiple of 10 with
// the word for the digit in ones place.
// For hundreds we use word for digit in hundreds place + 'hundred and'
// For 1000 we use word for digit in thousands place + 'thousand'
pub fn solve() -> Result<String> {
    let ans: usize = (1..=1000)
        .fold(0, |acc, i| {
            acc + num_to_words(i).split_whitespace().collect::<String>().len()
        });

    Ok(ans.to_string())
}

