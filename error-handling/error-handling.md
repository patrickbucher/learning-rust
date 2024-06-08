# Error Handling Q&A

## Incompatible Error Types: `map_err()`

- Q: My function returns `Result<T, E>`, but my error type is `F`.
- A: Use the `map_err()` method on the result.

Example:

```rust
use std::error::Error;
use std::fs::File;

fn main() {
    let handle = open_read("/etc/passwd");
    println!("{:?}", handle);
}

#[derive(Debug)]
struct Failure {
    cause: String,
}

fn open_read(path: &str) -> Result<File, Failure> {
    let file = File::open(path).map_err(|e| Failure {
        cause: e.to_string(),
    })?;
    Ok(file)
}
```

Output:

```text
Ok(File { fd: 3, path: "/etc/passwd", read: true, write: false })
```

## Option of a Reference, Reference to an Option: `as_ref()`

- Q: My function expects an `Option<&T>`, but I have an `&Option<T>`.
- A: Use the `as_ref()` method on the option.

Example:

```rust
fn main() {
    let decoration: &Option<char> = &Some('|');
    output(String::from("hello"), decoration.as_ref());
}

fn output(value: String, surround: Option<&char>) {
    match surround {
        Some(s) => println!("{s}{value}{s}"),
        None => println!("{value}"),
    }
}
```

Output:

```text
|hello|
```

## Unwrap an Option or a Result Using a Fallback Value: `unwrap_or()`

- Q: How can I process a collection of `Option<T>` or `Result<T, E>`, when I
  have a fallback value for `T`?
- A: Use the `unwrap_or()` method on the option or result.

Example:

```rust
fn main() {
    let inputs = ["123", "abc", "17", "", "1"];
    let sum: usize = inputs
        .iter()
        .map(|s| s.parse::<usize>())
        .map(|x| x.unwrap_or(0))
        .sum();
    println!("The sum of {:?} is {sum}.", inputs);
}
```

Output:

```text
The sum of ["123", "abc", "17", "", "1"] is 141.
```

## Partitioning a Collection of Results: `partition_result()` from Itertools

- Q: How can I partition the `T` and `E` values into two collections:
- A: Use `partition_result()` provided by [itertools](https://crates.io/crates/itertools).

Setup:

```bash
cargo add itertools
```

Example:

```rust
use itertools::Itertools;

fn main() {
    let inputs = ["123", "abc", "17", "", "1"];
    let (numbers, errors): (Vec<usize>, Vec<_>) =
        inputs.iter().map(|s| s.parse::<usize>()).partition_result();
    println!("numbers: {:?}", numbers);
    println!("errors: {:?}", errors);
}
```

Output:

```text
numbers: [123, 17, 1]
errors: [ParseIntError { kind: InvalidDigit }, ParseIntError { kind: Empty }]
```
