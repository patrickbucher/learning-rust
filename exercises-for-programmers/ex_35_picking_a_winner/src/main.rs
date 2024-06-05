use akshually::io::prompt_line;
use rand::{thread_rng, RngCore};

fn main() {
    let mut candidates: Vec<String> = Vec::new();
    loop {
        let candidate: Option<String> = prompt_line("Enter a name: ");
        match candidate {
            Some(c) => {
                if c.len() > 0 {
                    candidates.push(c)
                } else {
                    break;
                }
            }
            None => break,
        }
    }

    let mut rnd = thread_rng();
    let winner = &candidates[rnd.next_u32() as usize % candidates.len()];
    println!("The winner is... {winner}");
}
