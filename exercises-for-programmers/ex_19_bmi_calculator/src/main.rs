use akshually::io::prompt_line;

fn main() {
    let height: f32 = prompt_line("Your height in inches: ").expect("height");
    let weight: f32 = prompt_line("Your weight in pounds: ").expect("weight");
    let bmi = calculate_bmi(height, weight);
    println!("Your BMI is {bmi:1}.");
    if bmi < 18.5 {
        println!("You are underweight. Eat more.");
    } else if bmi >= 18.5 && bmi <= 25.0 {
        println!("You are within the ideal weight range.");
    } else {
        println!("You are overweight. You should see your doctor.");
    }
}

fn calculate_bmi(height: f32, weight: f32) -> f32 {
    (weight / (height * height)) * 703.0
}
