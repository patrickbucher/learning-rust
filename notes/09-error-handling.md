# Error Handling

Rust has no exceptions. Instead, it provides two ways of dealing with different
categories of errors: the `panic!` macro for unrecoverable errors, and the type
`Result<T, E>` for recoverable errors.

## Unrecoverable Errors: `panic!`

If an error condition occurs that cannot be recovered from, calling the
`panic!` macro (with a sensible error message, indicating the underlying issue)
will print the error message, undwind and clean up the stack and quit the
program:

```rust
fn main() {
    panic!("Something terrible happened!");
}
```
The output of the program will indicate the error:

		Finished dev [unoptimized + debuginfo] target(s) in 0.00s
		 Running `target/debug/tmp`
	thread 'main' panicked at 'Something terrible happened!', src/main.rs:2:5
	note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

The cause of the error is not always that easy to track down. In this case,
executing the program with backtrace enabled (as suggested in the output
above), the whole stack trace will be printed out upon panicking:

	$ RUST_BACKTRACE=1 cargo run
		Finished dev [unoptimized + debuginfo] target(s) in 0.01s
		 Running `target/debug/tmp`
	thread 'main' panicked at 'Something terrible happened!', src/main.rs:2:5
	stack backtrace:
	   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
				 at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:39
	   1: std::sys_common::backtrace::_print
				 at src/libstd/sys_common/backtrace.rs:71
	   2: std::panicking::default_hook::{{closure}}
				 at src/libstd/sys_common/backtrace.rs:59
				 at src/libstd/panicking.rs:197
	   3: std::panicking::default_hook
				 at src/libstd/panicking.rs:211
	   4: std::panicking::rust_panic_with_hook
				 at src/libstd/panicking.rs:474
	   5: std::panicking::begin_panic
				 at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/libstd/panicking.rs:408
	   6: tmp::main
				 at src/main.rs:2
	   7: std::rt::lang_start::{{closure}}
				 at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/libstd/rt.rs:64
	   8: std::panicking::try::do_call
				 at src/libstd/rt.rs:49
				 at src/libstd/panicking.rs:293
	   9: __rust_maybe_catch_panic
				 at src/libpanic_unwind/lib.rs:85
	  10: std::rt::lang_start_internal
				 at src/libstd/panicking.rs:272
				 at src/libstd/panic.rs:394
				 at src/libstd/rt.rs:48
	  11: std::rt::lang_start
				 at /rustc/a53f9df32fbb0b5f4382caaad8f1a46f36ea887c/src/libstd/rt.rs:64
	  12: main
	  13: __libc_start_main
	  14: _start

In order to find the erroneous code of your own, start reading the backtrace at
the top and follow along until you see a source file you've written on your own
(above: output line 6: `src/main.rs:2`).

Instead of unwinding and cleaning up the stack upon panicking, the program can
also abort immediately, leaving the cleanup task up to the operating system.
This can be configured in the respective `[profile]` section (debug, release)
in the project's `Cargo.toml` file:

```toml
[profile.release]
panic = 'abort'
```

## Recoverable Errors: `Result<T, E>`

Most error conditions do not require the program to stop, but only to take a
different path in the program logic. The result of a potentially failing
operation can be expressed using the `Result<T, E>` enum, which is defined as
such:

```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

An operation that couldl be executed as intended has the `Ok` variant set (with
any appropriate type), whereas a failing operation has the `Err` variant set
(with some error type).

For example, the method `File::open(filename)` returns a `Result<T, E>` with
`T=std::fs::File` set in case of success, and `E=std::io::Error` set in case of
an error.

The result can be handled using a `match` expression, panicking in the error
case:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Error opening file 'hello.txt': {:?}", error),
    };
}
```

Since there are different possible causes for a file not being able to be
opened, it is more appropriate to differentiate the error occured accordingly.
The `kind()` method of  the error can be used for this purpose, and the _match
guard_ syntax helps to prevent deeply indented error handling code blocks:

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("Error creating file 'hello.txt': {:?}", e),
        },
        Err(error) => panic!("Error opening file 'hello.txt': {:?}", error),
    };
}
```

The `ref` keyword ensures that the error value is not moved inside the `match`
arm, but referred to instead.

### `unwrap` and `expect`

The `unwrap` method of the `Result<T, E>` type is a shortcut for error
handling. If the `Ok` variant is set, `unwrap` returns the value set to the
`Ok` variant; if the `Err` variant is set, the program will cause a panic with
the default error message:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
The `expect` method works just like `unwrap`, but accepts a string parameter
indicating a custom error message:

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("error opening 'hello.txt'");
}
```

### Error Propagation

It is not always possible or desirable to deal with every error condition where
it originally occurs. Oftentimes, the callee does not know the context, and the
caller should decide on how to deal with the error. This technique is called
_error propagation_:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_str_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // propagate error
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e), // propagate error
    }
}

fn main() {
    match read_str_from_file() {
        Ok(_) => println!("ok"),
        Err(_) => panic!("not good"),
    }
}
```

This pattern is so common that Rust offers the shortcut operator `?` for
propagating error. The same code becomes much shorter using that syntactic
sugar:

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_str_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?; // ? instead of match
    let mut s = String::new();
    f.read_to_string(&mut s)?; // ? instead of match
    Ok(s)
}

fn main() {
    match read_str_from_file() {
        Ok(_) => println!("ok"),
        Err(_) => panic!("not good"),
    }
}
```

The `?` operator works as follows: If the expression in front of it evaluates
to the `Ok` variant of `Result`, the `Ok` variant is returned. If it evaluates
to the `Err` variant instead, the `Err` variant is returned.

One difference between the two implementations above is that `?` causes a call
to the `from` function of the `From` trait to convert the error type to the
defined return type (`io::Error` in this case).

The same function can be made even shorter by chaining the method calls after
the `?` operator:

```rust
fn read_str_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
```
The `?` operator can only be used in functions that return a `Result<T, E>`.

## Validation Types

Using validation functions throughout the code to prevent errors can be tedious
and lead to repeated code. Restrictions, say, on the value range of variables,
can be better expressed by types.

The builtin `u8` type, for example, is the perfect choice if discrete numbers
from 0 to 255 are the only acceptable values. However, there's not a builtin
type for every restriction needed. (Like a program for a lottery game, which
only accepts numbers between 1 and 45.) In this case, it's better to implement
a new custom type.

Implementing a custom validation type requires three things:

1. A private value variable with an appropriate range, enclosing the possible
   value range.
2. A public `new` method containing the validation code, which accepts a value
   parameter to be checked.
3. A public `value` method, returning the underlying value.

For the lottery game, a custom validation type can be implemented as follows:

```rust
pub struct Guess {
    value: u8,
}

impl Guess {
    pub fn new(value: u8) -> Guess {
        if value < 1 || value > 45 {
            panic!("acceptable range: 1..45");
        }
        Guess { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }
}

fn main() {
    Guess::new(10); // ok
    Guess::new(0); // not ok
    Guess::new(50); // not ok
}
```

## Using `panic!` or `Result<T, E>`?

Since there's no way to recover from a panic, working with `Result<T, E>` is
isually the more appropriate choice. The use of `panic!` (and the methods
`unwrap` and `expect`, which also cause a panic) should be restricted to the
following situations:

1. When a test case that needs to fail as a whole.
2. If an error is logically impossible, but the compiler cannot figure that
   out.
3. If the code could end up in a bad state that cannot be recovered from,
   leading to further undefined states. This usually happens when the caller is
   using an API the wrong way (contract violation). An API causing a panic must
   always mention that in its documentation.
4. For example code and prototypes.
