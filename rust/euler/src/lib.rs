pub fn fibonacci(i: u32) -> u32 {
    match i {
        0 => 0,
        1 => 1,
        2 => 2,
        _ => fibonacci(i - 1) + fibonacci(i - 2)
    }
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
}
