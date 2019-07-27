pub fn div(a: i32, b: i32) -> f32 {
    if b == 0 {
        panic!("division by zero");
    }
    a as f32 / b as f32
}

#[cfg(test)]
mod tests {
    use super::div;

    #[test]
    #[should_panic(expected="division by zero")]
    fn test_divide_by_zero() {
        div(3, 0);
    }
}
