pub fn staircase(n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1, // [1]
        2 => 2, // [1, 1], [2]
        3 => 4, // [1, 1, 1], [1, 2], [2, 1], [3]
        _ => staircase(n - 1) + staircase(n - 2) + staircase(n - 3),
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_staircase() {
        assert_eq!(staircase(0), 0);

        // [1]
        assert_eq!(staircase(1), 1);

        // [1, 1], [2]
        assert_eq!(staircase(2), 2);

        // [1, 1, 1], [2, 1], [1, 2], [3]
        assert_eq!(staircase(3), 4);

        /*
         * [1, 1, 1, 1]
         * [2, 1, 1], [1, 2, 1], [1, 1, 2]
         * [2, 2]
         * [3, 1], [1, 3]
         */
        assert_eq!(staircase(4), 7);

        /*
         * [1, 1, 1, 1, 1]
         * [2, 1, 1, 1], [1, 2, 1, 1], [1, 1, 2, 1], [1, 1, 1, 2]
         * [2, 2, 1], [2, 1, 2], [1, 2, 2]
         * [3, 1, 1], [1, 3, 1], [1, 1, 3]
         * [3, 2], [2, 3]
         */
        assert_eq!(staircase(5), 13);
    }
}
