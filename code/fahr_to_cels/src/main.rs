fn main() {
    println!("{}°F are {}°C", 100.0, fahr_to_cels(100.0));
    println!("{}°C are {}°F", 40.0, cels_to_fahr(40.0));
}

fn fahr_to_cels(f: f32) -> f32 {
    (f - 32.0) * 5.0 / 9.0
}

fn cels_to_fahr(c: f32) -> f32 {
    c * 9.0 / 5.0 + 32.0
}
