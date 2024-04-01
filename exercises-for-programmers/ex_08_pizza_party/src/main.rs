use std::io;

fn main() {
    println!("How many people?");
    let mut people = String::new();
    io::stdin()
        .read_line(&mut people)
        .expect("number of people");
    let people: u32 = people.trim().parse().expect("number");

    println!("How many pizzas do you have?");
    let mut pizzas = String::new();
    io::stdin()
        .read_line(&mut pizzas)
        .expect("number of pizzas");
    let pizzas: u32 = pizzas.trim().parse().expect("number");

    println!("How many slices per pizza?");
    let mut slices_per_pizza = String::new();
    io::stdin()
        .read_line(&mut slices_per_pizza)
        .expect("number of slices per pizza");
    let slices_per_pizza: u32 = slices_per_pizza.trim().parse().expect("number");

    let (slices_per_person, leftovers) = allocate(pizzas, slices_per_pizza, people);

    println!("{people} people with {pizzas} pizzas cut into {slices_per_pizza} slices");
    println!("Each person gets {slices_per_person} pieces of pizza.");
    println!("There are {leftovers} leftover pieces.");
}

fn allocate(wholes: u32, into_parts: u32, to_whom: u32) -> (u32, u32) {
    let parts = wholes * into_parts;
    (parts / to_whom, parts % to_whom)
}
