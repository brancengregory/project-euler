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
}
