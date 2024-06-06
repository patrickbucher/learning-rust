use akshually::io::prompt_line;

fn main() {
    let done = String::from("done");
    let mut response_times: Vec<usize> = Vec::new();
    loop {
        let input: Option<String> = prompt_line("Enter a number: ");
        let millis: usize = match input {
            Some(text) => {
                if text == done {
                    break;
                } else {
                    match text.parse() {
                        Ok(parsed) => parsed,
                        _ => continue,
                    }
                }
            }
            None => continue,
        };
        response_times.push(millis);
    }

    println!(
        "Numbers: {}",
        response_times
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    println!("The average is {}", mean(&response_times));
    println!("The minimum is {}", min(&response_times).unwrap());
    println!("The maximum is {}", max(&response_times).unwrap());
}

fn mean(xs: &Vec<usize>) -> f64 {
    let sum: usize = xs.iter().fold(0, |x, acc| acc + x);
    sum as f64 / xs.len() as f64
}

fn min(xs: &Vec<usize>) -> Option<usize> {
    xs.iter().min().copied()
}

fn max(xs: &Vec<usize>) -> Option<usize> {
    xs.iter().max().copied()
}

fn sd(xs: &Vec<usize>) -> f64 {
    let mean = mean(xs);
    let diff_sum = xs
        .iter()
        .map(|x| (*x as f64 - mean).powf(2.0))
        .fold(0.0, |x, acc| acc + x);
    (diff_sum / xs.len() as f64).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mean() {
        let xs = vec![10, 20, 30, 40, 50, 60];
        let expected = 35.0;
        let actual = mean(&xs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_min() {
        let xs = vec![9, 5, 6, 3, 2, 1];
        let expected = Some(1);
        let actual = min(&xs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_max() {
        let xs = vec![8, 5, 6, 3, 2, 9, 1];
        let expected = Some(9);
        let actual = max(&xs);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_sd() {
        let xs = vec![100, 200, 1000, 300];
        let actual = sd(&xs);
        assert!(actual > 353.0);
        assert!(actual < 354.0);
    }
}
