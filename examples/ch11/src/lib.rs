pub fn max(left: usize, right: usize) -> usize {
    if left > right {
        left
    } else {
        right
    }
}

pub fn tax(value: f64, tax_rate: f64) -> f64 {
    value * tax_rate
}

pub fn is_positive(value: f64) -> bool {
    value > 0.0
}

pub fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        panic!("cannot divide by zero");
    }
    a / b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let result = max(2, 3);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_tax() {
        let result = tax(100.0, 8.2);
        assert_ne!(result, 100.0);
    }

    #[test]
    fn test_is_positive() {
        assert!(is_positive(3.0));
    }

    #[test]
    fn test_not_is_positive() {
        assert!(!is_positive(-2.0));
    }

    #[test]
    #[should_panic(expected = "divide by zero")]
    fn test_divide_by_zero_panics() {
        divide(17.0, 0.0);
    }

    #[test]
    fn test_division() -> Result<(), String> {
        if divide(16.0, 2.0) == 8.0 {
            Ok(())
        } else {
            Err(String::from("16.0 divided by 2.0 should be 8.0"))
        }
    }
}
