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

In the Rust community, binary programs are structured as follows:

- The program is split up between `main.rs` and `lib.rs`.
- `main.rs` contains the command line parsing and some other configuration
  logic, as long as that logic is small.
- `lib.rs` contains the program's main logic, which is offered as a function
  called `run`.
- The `main` function calls `run` and deals with possible errors returned from
  there.

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

### Extracting the Parsing Logic

The parsing logic is better split up, especially as it grows with further
configuration parameters. A good approach is to extract these parts from
`main`:

- a struct, containing all the configuration field
- a function, parsing the initial arguments into such a struct

The latter function can be implemented as a constructor of the struct:

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

Cloning strings, as in the constructor of `Config`, is not very efficient, but
easy to implement. If only few objects are cloned, and only at the start of the
program like in the example above, cloning will not undermine the program's
performance in a critical manner.

## Reading and Processing a File

The content of a text file can be read into a string variable using the
`std::io::File` class and the traits from `std::io::predlude`:

```rust
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = &args[2]; // as above

    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error reading file");

    println!("content:\n{}", contents)
}
```

- TODO: iterate over lines, substring matching

## Error Handling

This constructor from above causes a panic if too few arguments are provided:

```rust
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
```

If only one argument is provided, `args` has no element at index `2`:

	$ cargo run query
	thread 'main' panicked at 'index out of bounds: the len is 2 but the index is 2'

### User-Friendly Error Messages

This error message is intended for programmers, not for users. This panic
message is a bit more user-friendly:

```rust
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
		// omitted
    }
}
```

	$ cargo run query
	thread 'main' panicked at 'not enough arguments'

### Error Message Instead of Panic

It is better, though, to let the `main` function decide what to do in case of
an error, than to shutdown the program with a panic right away. This version of
the constructor returns a `Result` with either the parsed `Config`, or an error
message if the parsing logic fails:

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
```

The actual result as to be wrapped in the `Ok` or the `Err` variant,
respectively. At the caller's side, the `Result` can be handled using the
`match` keyword ‒ or by the `unwrap_or_else` method of `Result`, which accepts
a closure for the error message:

```rust
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
}
```

The program is terminated with exit code `1` in case of a failure using the
`process::exit` function. The error message is sent to the standard output:

	$ cargo run query
	Problem parsing arguments: not enough arguments

TODO: p. 240 (extracting logic from main)

## Test-Driven Development

1. write test that fails for the expected reason
2. make the test compile and then pass
3. refactor the code without breaking the test
4. repeat from step 1 for the next part
