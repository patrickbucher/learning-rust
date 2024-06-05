use akshually::io::prompt_line;
use rand::{thread_rng, RngCore};

fn main() {
    let mut rnd = thread_rng();
    let answers: &[&str] = &["Yes", "No", "Maybe", "Ask again later."];
    loop {
        let _question: String = prompt_line("What's your question? ").unwrap();
        let answer = answers[rnd.next_u32() as usize % answers.len()];
        println!("{answer}");
    }
}
