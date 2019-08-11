# An I/O Project: Building a Command Line Program

Rust is fast, safe, works on different platforms, compiles to a single binary ‒
and therefore is a good fit for command line tools.

The example application `minigrep` searches for a string in a file:

```bash
$ minigrep string file.txt
```

It is created as a binary crate:

```bash
$ cargo new --bin minigrep
```

TODO: p. 233: separation of concerns

## Command Line Arguments

The standard library contains an iterator `std::env::args` over strings,
containing the command line arguments as passed to the program. (Alternatively,
`std::env::args_os` provides an iterator of `OsString` for dealing with invalid
unicode input.)

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
```

Including the parent `std::env` and referring to the iterator as `env::args` is
a good compromise between concise and readable code.

The iterator's `collect()` method turns the elements of the iterator into a
collection, which must be specified using a type annotation.

The first command line argument is always the path to the binary being invoked:

```bash
$ cargo run query file.txt
["target/debug/minigrep", "query", "file.txt"]
```

The arguments of interest can be stored in variables for later use:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
```

```bash
$ cargo run query file.txt
Searching for query
In file file.txt
```

## Reading and Processing a File

- open, read lines, iterate over lines, substring matching

## Error Handling

- panic, Result, unwrap_or_else (closure), Box<Error> trait object

## Exiting a Program

## Test-Driven Development

1. write test that fails for the expected reason
2. make the test compile and then pass
3. refactor the code without breaking the test
4. repeat from step 1 for the next part
