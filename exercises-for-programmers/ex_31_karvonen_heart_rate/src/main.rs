use akshually::io::prompt_line;

fn main() {
    let age: usize = prompt_line("Age: ").expect("no age entered");
    let resting_pulse: usize = prompt_line("Resting Pulse: ").expect("no resting pulse entered");
    println!("{:>8} | {:>7}", "Intensity", "Rate");
    println!("-------------------");
    for i in 55..=95 {
        if i % 5 != 0 {
            continue;
        }
        let target = compute_target_heart_rate(age, resting_pulse, i);
        println!("{i:>8}% | {target:>3} bpm")
    }
}

fn compute_target_heart_rate(age: usize, resting_pulse: usize, intensity: usize) -> usize {
    let intensity: f32 = intensity as f32 / 100 as f32;
    let target = (((220 - age) - resting_pulse) as f32 * intensity) + resting_pulse as f32;
    target.round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_heart_rate() {
        let age = 22;
        let resting_pulse = 65;
        let intensity_to_rate = [
            (55, 138),
            (60, 145),
            (65, 151),
            (85, 178),
            (90, 185),
            (95, 191),
        ];
        for (intensity, expected) in intensity_to_rate {
            let actual = compute_target_heart_rate(age, resting_pulse, intensity);
            assert_eq!(actual, expected);
        }
    }
}
