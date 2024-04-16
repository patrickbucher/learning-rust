use ch16::beavis_and_butthead;
use ch16::pythagoras;

fn main() {
    beavis_and_butthead(10, 100);
    for triangle in pythagoras(vec![(2.0, 3.0), (3.0, 4.0), (4.0, 5.0)]) {
        let (a, b, c): (f32, f32, f32) = triangle;
        println!("{a:.2}²+{b:.2}²={c:.2}²");
    }
}
