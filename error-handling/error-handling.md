# Error Handling

# Q&A

## Incompatible Error Types: `map_err()`

- Q: My function returns `Result<T, E>`, but my error type is `F`.
- A: Use the `map_err()` method on the result.

Example:

```rust
use std::error::Error;
use std::fs::File;

fn main() {
    let _handle = open_read("/etc/passwd");
}

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

## Unwrap an Option Using a Fallback Value: `unwrap_or()`

TODO: `unwrap_or()`
