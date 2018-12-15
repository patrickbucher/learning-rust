# Getting Started

## Setup

Setup using `rustup` (make sure to have a C linker and compiler installed):

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

Proceed with default options to get the latest stable release.

To update the environment variables, either log out and in again, or update
them manually:

```bash
$ source $HOME/.cargo/env
```

Check the version of the Rust compiler (`rustc`) and documentation (`rustdoc`):

```bash
$ rustc --version
rustc 1.31.0 (abe02cefd 2018-12-04)
$ rustdoc --version
rustdoc 1.31.0 (abe02cefd 2018-12-04)
```

Open the local documentation in a browser:

```bash
$ rustup doc
```

Update Rust once in a while:

```bash
$ rustup update
```

Uninstall Rust and `rustup` when no longer needed:

```bash
$ rustup self uninstall
```

Consider adding `rustfmt` for code formatting (since version 1.31):

```bash
$ rustup component add rustfmt
```

## Hello World (manually)

Create a file `hello.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

- `fn` is the keyword to create a function.
- `main` is the name of the function that gets executed first when the program
  is started.
- `()` is an empty parameter list, because `main` doesn't expect any
  parameters.
- `{` starts the function body.
- `println!` is a macro that prints a line of text to the standard output.
    - Macros end with a `!`, which helps to distinguish them from functions.
- `"Hello, world!"` is a string literal to be printed.
- `;` is needed at the end of every statement.
- `}` ends the function body.

Compile and run the program:

```bash
$ rustc hello.rs
$ ./hello
Hello, world!
```

## Hello World (using Cargo)

Check if `cargo` has been installed properly:

```bash
$ cargo --version
cargo 1.31.0 (339d9f9c8 2018-11-16)
```

Create a new binary project using `cargo` (as opposed to a library, which would
take the parameter `--lib` instead of `--bin`):

```bash
$ cargo new hello_world --bin
```

A directory `hello_world` has been created with the following contents:

- `src/`: the folder containing the source code for the project
- `src/main.rs`: the source code file containing the `main` function
- `.git` and `.gitignore`: files for Git (define another version control system
  using the `--vcs` parameter)
- `Cargo.toml`: the file containing the project configuration:

```toml
[package]
name = "hello_world"
version = "0.1.0"
authors = ["Patrick Bucher <patrick.bucher@stud.hslu.ch>"]
edition = "2018"

[dependencies]
```

TOML stands for «Tom's Obvious, Minimal Language». The `authors` information
is taken from the local Git configuration. The project doesn't have any
dependencies yet.

Build and execute (for testing):

```bash
$ cargo build
$ ./target/debug/hello_world
Hello, world!
```

Build and execute (for release with optimized binary):

```bash
$ cargo build --release
$ ./target/release/hello_world
Hello, world!
```

A new file `Cargo.lock` is created to keep track of the dependencies versions.

Build and run in one step:

```bash
$ cargo run
Hello, world!
```

Only check the source code without creating a binary (faster):

```bash
$ cargo check
```
