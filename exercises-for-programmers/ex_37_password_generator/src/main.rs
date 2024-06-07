use akshually::io::prompt_line;
use rand::{thread_rng, RngCore};

fn main() {
    let n_min_length: usize = prompt_line("What's the minimum length? ").unwrap();
    let n_special_chars: usize = prompt_line("How many special characters? ").unwrap();
    let n_numbers: usize = prompt_line("How many numbers? ").unwrap();

    let mut letters: Vec<_> = ('a'..='z').collect();
    let mut letters_upper: Vec<_> = ('A'..='Z').collect();
    letters.append(&mut letters_upper);
    let special_chars = vec![',', '.', ';', ':', '<', '>', '!', '@', '#', '4', '%', '*'];
    let numbers: Vec<_> = ('0'..='9').collect();

    let mut rnd = thread_rng();
    let mut rand_chars: Vec<char> = Vec::new();
    for i in 0..n_special_chars {
        rand_chars.push(special_chars[rnd.next_u32() as usize % special_chars.len()]);
    }
    for i in 0..n_numbers {
        rand_chars.push(numbers[rnd.next_u32() as usize % numbers.len()]);
    }
    for i in 0..(n_min_length - n_special_chars - n_numbers) {
        rand_chars.push(letters[rnd.next_u32() as usize % letters.len()]);
    }

    let password = shuffle(&rand_chars);
    let password: String = password.into_iter().collect();
    println!("Your password is\n{password}");
}

fn shuffle(chars: &Vec<char>) -> Vec<char> {
    let mut chars: Vec<char> = chars.clone();
    let mut shuffled: Vec<char> = Vec::new();
    let mut rnd = thread_rng();
    while chars.len() > 0 {
        let i = rnd.next_u32() as usize % chars.len();
        shuffled.push(chars[i]);
        chars.remove(i);
    }
    shuffled
}
