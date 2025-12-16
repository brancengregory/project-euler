use num::BigUint;

pub fn fibonacci(i: u64) -> u64 {
    match i {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => fibonacci(i - 1) + fibonacci(i - 2)
    }
}

pub fn triangle_numbers(n: u64) -> Vec<u64> {
    (1..).scan(1, |state, i| {
        let current_value = *state;
        *state = current_value + i + 1;
        Some(current_value)
    })
        .take(n as usize)
        .collect()
}

pub fn triangle_iter() -> impl Iterator<Item = u64> {
    (1..).scan(1, |state, i| {
        let current_value = *state;
        *state = current_value + i + 1;
        Some(current_value)
    })
}

pub fn primes_le(n: u64) -> Vec<u64> {
    let nums: Vec<u64> = (2..=n).collect();
    let mut is_prime: Vec<bool> = vec![true; nums.len()];

    for i in 0..(n.isqrt()) {
        if is_prime[i as usize] {
            let mut x = nums[i as usize].pow(2);

            while x <= (nums.len() + 1) as u64 {
                is_prime[(x - 2) as usize] = false;
                x += nums[i as usize];
            }
            
        }
    }

    nums.into_iter().enumerate()
        .filter_map(|(i, x)| {
            if is_prime[i] {
                Some(x)
            } else {
                None
            }
        })
        .collect()
}

pub fn primes(n: u64) -> Vec<u64> {
    let upper_bound: u64 = if n < 6 {
        13
    } else {
        // Use Rossers theorem to calculate upper bound for large n
        let n = n as f64;
        (n * (n.ln() + n.ln().ln())) as u64
    };

    primes_le(upper_bound).into_iter().take(n as usize).collect()
}

pub fn nth_prime(n: u64) -> u64 {
    *primes(n).last().unwrap()
}

pub fn reverse_int(n: u64) -> u64 {
    std::iter::successors(Some(n), |&x| (x >= 10).then_some(x / 10))
        .map(|x| x % 10)
        .fold(0, |acc, digit| acc * 10 + digit)
}

pub fn is_palindrome(n: u64) -> bool {
    let rev = reverse_int(n);
    n == rev
}

pub fn sum_squares(n: u64) -> u64 {
    (1..=n).map(|x| x.pow(2)).sum()
}

pub fn squared_sum(n: u64) -> u64 {
    (1..=n).sum::<u64>().pow(2)
}

pub fn divisors(i: u64) -> Vec<u64> {
    let limit = i.isqrt();

    let mut divisors: Vec<u64> = Vec::new();

    for x in 1..=limit {
        if i.is_multiple_of(x) {
            divisors.push(x);

            if x * x != i {
                divisors.push(i / x);
            }
        }
    }

    divisors.sort_unstable();
    divisors
}

pub fn n_divisors(i: u64) -> u64 {
    let limit = i.isqrt();

    (1..=limit)
        .filter(|&x| i.is_multiple_of(x))
        .map(|x| if x * x == i { 1 } else { 2 })
        .sum()
}

pub fn sum_divisors(i: u64) -> u64 {
    let divisors = divisors(i);
    divisors.iter().sum()
}

pub fn proper_divisors(i: u64) -> Vec<u64> {
    let divisors = divisors(i);
    if divisors.len() > 1 {
        return divisors[0..(divisors.len() - 1)].to_vec()
    }

    return divisors
}

pub fn sum_proper_divisors(i: u64) -> u64 {
    let proper_divisors = proper_divisors(i);
    proper_divisors.iter().sum()
}

pub fn is_amicable(i: u64) -> bool {
    let proper_divisor_sum = sum_proper_divisors(i);
    i != proper_divisor_sum && i == sum_proper_divisors(proper_divisor_sum)
}

pub fn factorial(i: u64) -> BigUint {
    (1..=i).product()
}

pub fn n_digits(i: u64) -> u64 {
    match i {
        0 => 1,
        _ => (i.ilog10() + 1).into()
    }
}

pub fn num_to_words(i: u64) -> String {
    let ones = vec!["zero", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine"]; 
    let teens = vec!["ten", "eleven", "twelve", "thirteen", "fourteen",
        "fifteen", "sixteen", "seventeen", "eighteen", "nineteen"];
    let tens = vec!["twenty", "thirty", "forty", "fifty",
        "sixty", "seventy", "eighty", "ninety"];
 
    let mut words: Vec<&str> = Vec::new();

    if i < 10 {
        return ones[i as usize].to_string()
    } else if i < 20 {
        return teens[(i - 10) as usize].to_string()
    } else if i < 100 {
        let ones_digit = i % 10;
        
        if ones_digit != 0 {
            words.push(ones[ones_digit as usize]);
            words.push(" ");
        }

        let tens_digit = i / 10; 
        words.push(tens[(tens_digit - 2) as usize]);

        return words.into_iter().rev().collect::<String>()
    } else if i < 1000 {
        let ones_digit = i % 10;
        let tens_digit = (i / 10) % 10;
        let hundreds_digit = (i / 100) % 10;

        if tens_digit == 1 {
            words.push(teens[(ones_digit) as usize]);
            words.push(" and ");
            words.push("hundred");
            words.push(" ");
            words.push(ones[hundreds_digit as usize]);

            return words.into_iter().rev().collect::<String>()
        } 
        if ones_digit == 0 && tens_digit == 0 {
            words.push("hundred");
            words.push(" ");
            words.push(ones[hundreds_digit as usize]);

            return words.into_iter().rev().collect::<String>()
        } else if ones_digit == 0 {
            words.push(tens[(tens_digit - 2) as usize]);
            words.push(" and ");
            words.push("hundred");
            words.push(" ");
            words.push(ones[hundreds_digit as usize]);

            return words.into_iter().rev().collect::<String>()
        } else if tens_digit == 0 {
            words.push(ones[ones_digit as usize]);
            words.push(" and ");
            words.push("hundred");
            words.push(" ");
            words.push(ones[hundreds_digit as usize]);

            return words.into_iter().rev().collect::<String>()
        } else {
            words.push(ones[ones_digit as usize]);
            words.push(" ");
            words.push(tens[(tens_digit - 2) as usize]);
            words.push(" and ");
            words.push("hundred");
            words.push(" ");
            words.push(ones[hundreds_digit as usize]);

            return words.into_iter().rev().collect::<String>()
        }
    } else if i == 1000 {
        words.push("one thousand");
        return words.into_iter().collect::<String>()
    }

    panic!("Words not implemented for numbers greater than 1,000");
}

pub fn num_to_digits(n: u64) -> Vec<u8> {
    let mut digits: Vec<u8> = Vec::new();
    let mut num = n;

    if num == 0 {
        return vec![0]
    }
    
    while num > 0 {
        digits.push((num % 10) as u8);
        num /= 10;
    }

    digits.reverse();

    digits
}

pub fn digits_to_num(digits: &[u8]) -> u64 {
    if digits.is_empty() {
        panic!("No digits provided")
    }
    
    digits.iter().rev().enumerate().fold(0_u64, |acc, (i, &x)| {
        acc + (x as u64 * 10_u64.pow(i as u32))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(0), 0);
        assert_eq!(fibonacci(1), 1);
        assert_eq!(fibonacci(2), 2);
        assert_eq!(fibonacci(3), 3);
        assert_eq!(fibonacci(10), 89);
    }

    #[test]
    fn test_triangle_numbers() {
        assert_eq!(triangle_numbers(0), vec![]);
        assert_eq!(triangle_numbers(6), vec![1, 3, 6, 10, 15, 21]);
    }

    #[test]
    fn test_primes_le() {
        assert_eq!(primes_le(12), vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_primes() {
        assert_eq!(primes(5), vec![2, 3, 5, 7, 11]);
        assert_eq!(primes(20), vec![
            2, 3, 5, 7, 11, 13, 17,
            19, 23, 29, 31, 37, 41,
            43, 47, 53, 59, 61, 67, 71
        ]);
    }

    #[test]
    fn test_nth_prime() {
        assert_eq!(nth_prime(12), 37);
    }

    #[test]
    fn test_reverse_int() {
        assert_eq!(reverse_int(999888), 888999);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(9987899), true);
        assert_eq!(is_palindrome(123456), false);
    }

    #[test]
    fn test_sum_squares() {
        assert_eq!(sum_squares(10), 385);
    }

    #[test]
    fn test_squared_sum() {
        assert_eq!(squared_sum(10), 3025);
    }

    #[test]
    fn test_divisors() {
        assert_eq!(divisors(10), vec![1, 2, 5, 10]);
    }

    #[test]
    fn test_n_divisors() {
        assert_eq!(n_divisors(10), 4);
    }

    #[test]
    fn test_sum_divisors() {
        assert_eq!(sum_divisors(10), 18);
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(10), BigUint::from(3628800_u32));
    }

    #[test]
    fn test_n_digits() {
        assert_eq!(n_digits(0), 1);
        assert_eq!(n_digits(9), 1);
        assert_eq!(n_digits(12), 2);
        assert_eq!(n_digits(124), 3);
        assert_eq!(n_digits(1245), 4);
    }

    #[test]
    fn test_num_to_words() {
        assert_eq!(num_to_words(0), "zero".to_string());
        assert_eq!(num_to_words(3), "three".to_string());
        assert_eq!(num_to_words(10), "ten".to_string());
        assert_eq!(num_to_words(12), "twelve".to_string());
        assert_eq!(num_to_words(30), "thirty".to_string());
        assert_eq!(num_to_words(52), "fifty two".to_string());
        assert_eq!(num_to_words(200), "two hundred".to_string());
        assert_eq!(num_to_words(212), "two hundred and twelve".to_string());
        assert_eq!(num_to_words(234), "two hundred and thirty four".to_string());
        assert_eq!(num_to_words(1000), "one thousand".to_string());
    }

    #[test]
    fn test_num_to_digits() {
        assert_eq!(num_to_digits(0), vec![0]);
        assert_eq!(num_to_digits(100), vec![1, 0, 0]);
        assert_eq!(num_to_digits(124654116180), vec![1, 2, 4, 6, 5, 4, 1, 1, 6, 1, 8, 0]);
    }

    #[test]
    fn test_digits_to_num() {
        assert_eq!(digits_to_num(&vec![0]), 0);
        assert_eq!(digits_to_num(&vec![1, 0, 0]), 100);
        assert_eq!(digits_to_num(&vec![1, 2, 4, 6, 5, 4, 1, 1, 6, 1, 8, 0]), 124654116180);
    }
}
