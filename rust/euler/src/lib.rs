pub fn fibonacci(i: u64) -> u64 {
    match i {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => fibonacci(i - 1) + fibonacci(i - 2)
    }
}

pub fn primes_le(n: u64) -> Vec<u64> {
    let nums: Vec<u64> = (2..=n).collect();
    let mut is_prime: Vec<bool> = vec![true; nums.len()];

    for i in 0..n.isqrt() {
        if is_prime[i as usize] {
            let mut x = nums[i as usize].pow(2);

            while x <= (nums.len() + 2) as u64 {
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

pub fn reverse_int(n: u64) -> u64 {
    std::iter::successors(Some(n), |&x| (x >= 10).then_some(x / 10))
        .map(|x| x % 10)
        .fold(0, |acc, digit| acc * 10 + digit)
}

pub fn is_palindrome(n: u64) -> bool {
    let rev = reverse_int(n);
    n == rev
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
    fn test_primes() {
        assert_eq!(primes_le(12), vec![2, 3, 5, 7, 11]);
    }

    #[test]
    fn test_reverse_int() {
        assert_eq!(reverse_int(999888), 888999);
    }
}
