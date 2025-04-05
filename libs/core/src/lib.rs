pub fn is_odd<T>(n: T) -> bool
where
    T: Into<String>,
{
    let n_str = n.into();
    match n_str.parse::<u64>() {
        Ok(num) => num % 2 == 1,
        Err(_) => false,
    }
}

pub fn is_even<T>(n: T) -> bool
where
    T: Into<String>,
{
    let n_str = n.into();
    match n_str.parse::<u64>() {
        Ok(num) => num % 2 == 0,
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_odd_core() {
        assert!(is_odd(1.to_string()));
        assert!(!is_odd(2.to_string()));
    }

    #[test]
    fn test_is_even_core() {
        assert!(!is_even(1.to_string()));
        assert!(is_even(2.to_string()));
    }
}
