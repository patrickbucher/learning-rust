# Guessing Game

## User Input

Bring the standard input/output library into scope:

```rust
use std::io;
```

Create a new `String` and bind it to a mutable variable:

```rust
let mut s = String::new();
```

Read a line of user input into a `String`, fail with an error message if it
didn't work; print the input otherwise:

```rust
let mut input = String::new();
io::stdin()
    .read_line(&mut input)
    .expect("reading string failed"); // stops program execution
println!("{}", input);
```

Convert a the `String` from above into a number (unsigned 32-bit integer), fail
with an error message if it didn't work; print the converted number otherwise:

```rust
let input: u32 = guess.trim().parse().expect("not a number");
println!("{}", input);
```

The second variable `input` _shadows_ the first variable with the same name.

## Random Numbers

In order to generate random numbers, the [`rand`
crate](https://crates.io/crates/rand) needs to be added as a dependency in
`Cargo.toml`:

```toml
[dependencies]
rand = "0.3.14"
```

The version indicator `"0.3.14"` is shorthand for `"^0.3.14"` and means: any
version with a public API compatible to version 0.3.14.

As soon as the project is built (using `cargo build`), all the dependencies are
resolved (`libc` as a dependency from `rand`, for example), and the working
version configuration is written to a file `Cargo.lock`. This version
configuration is used for the next build, unless the dependencies in the
`Cargo.toml` file are updated, or the command `cargo update` is executed. The
latter option will update the `rand` dependency.

Build and view the documentation of all the project's dependencies, including
the `rand` crate, in the web browser:

```bash
$ cargo doc --open
```

Make the `rand` crate available in the project:

```rust
extern crate rand;
```

Import the `Rng` trait, which is needed to create random numbers within a
certain range:

```rust
use rand::Rng;
```

Create a random number within a range (lower bound inclusive, upper bound
exclusive):

```rust
let min = 1;
let max = 101;
let number = rand::thread_rng().gen_range(min, max);
println!("Random number 1..100: {}", number);
```

## Compare Numbers

Make the `Ordering` enum available, which covers all the possible results of a
comparison:

```rust
use std::cmp::Ordering;
```

Compare two numbers which one another using pattern matching:

```rust
let a = 3;
let b = 5;
match a.cmp(&b) {
    Ordering::Less => println!("a<b"),
    Ordering::Greater => println!("a>b"),
    Ordering::Equal => println!("a=b"),
}
```

## Loops and User Input

Request user input until a number is entered using a infinite loop:

```rust
loop {
    let mut input = String::new();
    println!("enter a number");
    match io::stdin().read_line(&mut input) {
        Result::Ok(_) => (), // do nothing
        Result::Err(_) => continue, // once again
    }
    let input: u32 = match input.trim().parse() {
        Ok(num) => num, // parsed input as the match expression's result
        Err(_) => {
            println!("not a number");
            continue; // once again
        }
    };
    // do something sensible with the number entered
    break;
}
```
