use std::io;

fn main() {
    let mut noun = String::new();
    println!("Enter a noun:");
    io::stdin().read_line(&mut noun).expect("noun needed");

    let mut verb = String::new();
    println!("Enter a verb:");
    io::stdin().read_line(&mut verb).expect("verb needed");

    let mut adjective = String::new();
    println!("Enter a adjective:");
    io::stdin()
        .read_line(&mut adjective)
        .expect("adjective needed");

    let mut adverb = String::new();
    println!("Enter a adverb:");
    io::stdin().read_line(&mut adverb).expect("adverb needed");

    let v = verb.trim();
    let adj = adjective.trim();
    let n = noun.trim();
    let adv = adverb.trim();
    println!("Do you {v} your {adj} {n} {adv}? That's hilarious!");
}
