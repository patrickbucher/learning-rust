extern crate mememo;

use mememo::Median;

#[test]
fn test_mean_vec() {
    let numbers = vec![1, 2, 3];
    const EXPECTED: f64 = 2.0;
    let got = mememo::mean(&numbers);
    assert_eq!(got, EXPECTED);
}

#[test]
fn test_median_odd() {
    let numbers = vec![1, 2, 3, 4, 5];
    const EXPECTED: i32 = 3;
    match mememo::median(&numbers) {
        Median::MiddleSingle(got) => assert_eq!(got, EXPECTED),
        _ => panic!("wrong median of vector with odd number of elements"),
    };
}

#[test]
fn test_median_even() {
    let numbers = vec![1, 2, 3, 4];
    const EXPECTED: f64 = 2.5;
    match mememo::median(&numbers) {
        Median::MiddleTwoMean(got) => assert_eq!(got, EXPECTED),
        _ => panic!("wrong median of vector with even number of elements"),
    };
}

#[test]
fn test_mode() {
    let numbers = vec![1, 2, 3, 1, 2, 1];
    const EXPECTED: i32 = 1;
    let got = mememo::mode(&numbers);
    assert_eq!(got, EXPECTED)
}
