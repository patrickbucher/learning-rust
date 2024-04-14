use akshually::prompt_line;

fn main() {
    let weight: f32 = prompt_line("Weight (lbs): ").expect("weight");
    let gender: char = prompt_line("Gender [m/w]: ").expect("gender");
    let drinks: u8 = prompt_line("Number of Drinks: ").expect("drinks");
    let size: f32 = prompt_line("Size Drinks (oz): ").expect("size");
    let alc_vol: f32 = prompt_line("Alcohol Vol. %: ").expect("alc_vol");
    let hours: u8 = prompt_line("Hours since last drink: ").expect("hours");

    let (bac, legal) = blood_alcohol(weight, gender, drinks, size, alc_vol, hours);

    println!("Your BAC is {bac:.3}");
    if legal {
        println!("It is legal for you to drive.");
    } else {
        println!("It is not legal for you to drive.");
    }
}

fn blood_alcohol(
    weight: f32,
    gender: char,
    drinks: u8,
    size: f32,
    alc_vol: f32,
    hours: u8,
) -> (f32, bool) {
    let alcohol_consumed = drinks as f32 * size * (alc_vol / 100.0);
    let ratio: f32 = if gender == 'm' { 0.73 } else { 0.66 };
    let bac = alcohol_consumed * 5.14 / weight * ratio - 0.15 * hours as f32;
    let legal = bac < 0.08;
    if bac < 0.0 {
        (0.0, legal)
    } else {
        (bac, legal)
    }
}
