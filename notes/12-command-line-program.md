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

## Refactoring

Once the command line parameters are wrapped up in a handy config object, the
program logic can be extracted from the `main` function and put into a `run`
function:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config);
}

fn run(config: Config) {
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("error reading file");
    println!("content:\n{}", contents)
}
```

The `run` function causes panics, which takes away control from the `main`
function. To give that control back, this `run` implementation returns a
result, wrapping possible errors up in a `Box`. Instead of calling `except` on
the expressions that return a `Result` by themselves, the `?` operator can be
used:

```rust
use std::error::Error;

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    println!("content:\n{}", contents);
	Ok(())
}
```

The `Ok` variant only wraps the unit value `()`. In `main`, the error message
can be used accordingly:

```rust
fn main() {
	//  omitted

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

Unlike `Config::new`, `run` does not a value that can be unwrapped. So here the
`if let` construct is used to handle possible errors.

The extracted parts ‒ the `run` function and the `Config` struct with its `new`
method ‒ can now be moved into a separated library crate, which makes it easier
to reuse and test that code:

```rust
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
	// omitted
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
	// omitted
}
```

The `Config` struct, its fields, its `new` method, and the `run` function need
to be public, so that they can be used from the stripped-down `main.rs`:

```rust
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
```

The extracted functionality now has to be included as an extern crate
`minigrep`.

## Test-Driven Development

The program will be finished from here in a process called Test-Driven
Development (TDD), which works as follows:

1. Write test that fails for the expected reason.
2. Make the test compile and then pass.
3. Refactor the code without breaking the test.
4. Repeat from step 1 for the next part.

### Failing Test Case

First, a (failing) test case ‒ `one_result` ‒ is added to `lib.rs`:

```rust
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

### Dummy Implementation

This code does not even compile, because the `search` function is missing. A
dummy implementation, returning an empty vector, makes the code compile, but
still lets the test fail:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}
```

The `search` function needs to annotate a lifetime: The strings in the
resulting vectors are pulled out of the `contents` parameter, and therefore
must life as long as that underlying string object.

### Real Implementation

With this implementation of `search`, the test will pass:

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

The text is analyzed line by line using the `lines` method of the `contents`
string. The `contains` method returns true, if the given substring `query` is
part of the string (the current line, that is). Matching lines are added to the
`results` vector, which is returned from the function.

The test is now passing, but the program does not make use of `search` yet,
which can be done as such:

```rust
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}
```

The program now yields only the matching lines:

```bash
$ cargo run nobody poem.txt
Searching for nobody
In file poem.txt
I'm nobody! Who are you?
Are you nobody, too?
```

## Environment Variable for Case Insensitive Search

`minigrep` needs an option to search through files in a case-insensitive
manner. To make it possible for the user to make that option permanent instead
of providing it with every invocation of the program, an environment variable
should be introduced instead of a command line argument.

Continuing the test-driven approach from before, an additional (failing) test
is added to the test module of `lib.rs`:

```rust
#[test]
fn case_insensitive() {
	let query = "rUsT";
	let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

	assert_eq!(
		vec!["Rust:", "Trust me."],
		search_case_insensitive(query, contents)
	);
}
```

For the implementation, both the query and every line to be tested are
converted to lowercase:

```rust
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}
```

The slice `query` is converted to a `String` by the `to_lowercase` method.
Therefore, it has to be passed as `&query` into the `contains` method.

Both test cases now run successfully.

### Additional Option

The new case insensitive option will be held by the `Config` struct:

```rust
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
```

Depending on its value, either the `search` (case sensitive) or the
`search_case_insensitive` function will be called from run:

```rust
pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}
```

If the environment variable `CASE_INSENSITIVE` is set (to any value), the
according option should be set to `true`. The `var` function of the `env`
module retrieves the value of the given environment variable and returns a
`Result`. This logic belongs to the constructor of `Config`:

```rust
use std::env;

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
```

The `is_err` method returns true if the environment variable `CASE_INSENSITIVE`
is not set, in which case a case-sensitive search has to be performed. In all
other cases, a case-insensitive search was chosen by the user.

The behaviour of the program now can be changed by setting the
`CASE_INSENSITIVE` environment variable:

```bash
$ cargo run To poem.txt
Searching for To
In file poem.txt
To tell your name the livelong day
To an admiring bog!

$ CASE_INSENSITIVE=1 cargo run To poem.txt
Searching for To
In file poem.txt
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```

The second invocation yields two additional lines, which matches the specified behaviour.

### Printing to Standard Error

The program prints debug and error messages to standard output (`stdout`), just
like the matching lines. In order to process the desired output -- the matching
lines -- further with an other program, or to store it in a file, the debug and
error messages should be sent to standard error (`stderr`) instead.

The `eprintln!` macro works just like the `println!` macro, but it prints to
`stderr` instead of `stdout`. Since only the `main` function prints debug and
error messages, it is the only place due for refactoring:

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    eprintln!("Searching for {}", config.query);
    eprintln!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
```

The debug and error messages can now be disposed of by forwarding them do
`/dev/null`, for example:

```bash
$ cargo run To poem.txt 2> /dev/null
To tell your name the livelong day
To an admiring bog!

$ CASE_INSENSITIVE=1 cargo run To poem.txt 2> /dev/null
Are you nobody, too?
How dreary to be somebody!
To tell your name the livelong day
To an admiring bog!
```
