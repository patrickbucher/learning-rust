# Writing Automated Tests

Rust has built-in support for automated tests. A test module and a test
function is automatically generated for every new library crate:

	$ cargo new maths --lib

The generated code:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

The test module is annotated with `[#cfg(test)]`, and the test function with
`[#test]`. The function body tests if `2 + 2` equals `4`, which always holds
true, so the test case passes when run using `cargo test` (excerpt of the
output):

	$ cargo test

	running 1 test
	test tests::it_works ... ok

	test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

A test fails if a test function (or one of the function it calls) panics:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_panics() {
        panic!("nothing works yet");
    }
}
```

Output of `cargo test` (excerpt):

	running 1 test
	test tests::it_panics ... FAILED

	failures:

	---- tests::it_panics stdout ----
	thread 'tests::it_panics' panicked at 'nothing works yet', src/lib.rs:5:9
	note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


	failures:
		tests::it_panics

	test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

## Assertions

The `assert!` macro ensures that an expression evaluates to `true`. If the
expression evaluates to `false`, the `panic!` macro is called and the test case
fails.

This test case checks the return value of the `add` function. The test module
needs to import the function from its super module:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        // arrange
        let a = 7;
        let b = 3;
        let expected_sum = 10;

        // act
        let actual_sum = add(a, b);

        // assert
        assert!(actual_sum == expected_sum);
    }
}
```

The test function is built up using the "AAA" pattern, which stands for:

1. Arrange: set up the test data
2. Act: run the code to be tested
3. Assert: check the outcomes against the expectations

The macros `assert_eq!` is especially helpful for comparisons. It takes two
parameters, called `left` and `right`, checks them for equality.

This broken implementation of the `subtract` function shows the difference
between `assert!` and `assert_eq!`:

```rust
pub fn sub(a: i32, b: i32) -> i32 {
    b - a // broken: must be a - b
}

#[cfg(test)]
mod tests {
    use super::sub;

    #[test]
    fn test_subtract1() {
        assert!(7 == sub(10, 3))
    }

    #[test]
    fn test_subtract2() {
        assert_eq!(7, sub(10, 3))
    }
}
```

Output of `cargo test`:

	running 2 tests
	test tests::test_subtract1 ... FAILED
	test tests::test_subtract2 ... FAILED

	failures:

	---- tests::test_subtract1 stdout ----
	thread 'tests::test_subtract1' panicked at 'assertion failed: 7 == sub(10, 3)', src/lib.rs:30:9
	note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

	---- tests::test_subtract2 stdout ----
	thread 'tests::test_subtract2' panicked at 'assertion failed: `(left == right)`
	  left: `7`,
	 right: `-7`', src/lib.rs:35:9


	failures:
		tests::test_subtract1
		tests::test_subtract2

	test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out

In `subtract1`, using the `assert!` macro, the unevaluated expression is shown.
In `subtract2`, using the `assert_eq!` macro, the evaluated `left` and `right`
expressions are shown, making error detection arguably easier.

The `assert_ne!` macro ensures that the `left` and `right` expression are _not_
equals. Sometimes, it is not possible to say what the outcome of an operation
should be, but it's well possible to say what the outcome must _not_ be. 

## Custom Failure Messages

The macros `assert!`, `assert_eq!` and `assert_ne!` take an optional third
parameter: a custom error message that is printed in case the assertion fails:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b // correct
}

pub fn sub(a: i32, b: i32) -> i32 {
    b - a // broken
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::sub;

    #[test]
    fn test_add() {
        assert_eq!(10, add(7, 3), "adding 3 to 7 failed")
    }

    #[test]
    fn test_subtract() {
        assert_eq!(7, sub(10, 3), "subtracting 3 from 10 failed")
    }
}
```

Output (excerpt):

	running 2 tests
	test tests::test_add ... ok
	test tests::test_subtract ... FAILED

	failures:

	---- tests::test_subtract stdout ----
	thread 'tests::test_subtract' panicked at 'assertion failed: `(left == right)`
	  left: `7`,
	 right: `-7`: subtracting 3 from 10 failed', src/lib.rs:21:9
	note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.


	failures:
		tests::test_subtract

	test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out

	error: test failed, to rerun pass '--lib'

The custom error message of the failing `test_subtract` case is shown in the
output.

## Testing for Panics

As stated in the chapter about error handling, a panic is a valid response to a
public interface that is used the wrong way. The `#[should_panic]` annotation
helps testing such cases: The test case passes if a panic is caused upon
execution, and fails otherwise.

This `div` function panics if the given divisor is equal to zero, and the panic
is expected in the test function:

```rust
pub fn div(a: i32, b: i32) -> f32 {
    if b == 0 {
        panic!("division by zero");
    }
    a as f32 / b as f32
}

#[cfg(test)]
mod tests {
    use super::div;

    #[test]
    #[should_panic]
    fn test_divide_by_zero() {
        div(3, 0);
    }
}
```

The test passes, but it would also pass if the panic happened for a different
reason. For such cases, the optional `expected` parameter of the `should_panic`
annotation helps to ensure that the underlying code panics for the right reason
by checking if the `expected` parameter is a substring of the actual panic
message:

```rust
#[test]
#[should_panic(expected="division by zero")]
fn test_divide_by_zero() {
	div(3, 0);
}
```

This approach is somewhat shaky, because panic messages can change as time
goes. The `expected` parameter therefore must be chosen to be neither too
specific (danger of breaking with changes) nor too general (danger of accepting
wrong panic messages).

## Test Execution

### Arguments

The command `cargo test` compiles the project in test mode and runs the
resulting test binary. Command line parameters can be indicated both for cargo
and the test binary. For the latter case, parameters following the `--`
separator are sent to the test binary:

	cargo test --foo # send parameter --foo to cargo
	cargo test -- --bar #  send parameter --bar to the test binary
	cargo test --foo -- --bar # send --foo to cargo, --bar to the test binary

### Parallel Execution

By default, multiple tests run in multiple threads: one test per thread, that
is. The order of test execution is not deterministic, and therefore tests
should not depend on one another. However, the number of threads can be defined
using the `--test-threads` parameter, which can be set to `1` for sequential
execution.

	cargo test -- --test-threads=1

The standard output of passing tests won't be shown in the test result.
Consider this two test cases:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn passing_test() {
        println!("is 2 + 2 = 4?");
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn failing_test() {
        println!("is 2 + 2 = 5?");
        assert_eq!(2 + 2, 5);
    }
}
```

### Output

When run with `cargo test`, only the `println!` output of the failing test is
shown in the `stdout` section of the test output:

	---- tests::failing_test stdout ----
	is 2 + 2 = 5?
	thread 'tests::failing_test' panicked at 'assertion failed: `(left == right)`
	  left: `4`,
	 right: `5`', src/lib.rs:12:9
	note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.

When run with `cargo test -- --nocapture` instead, also the output of the
passing test is shown:

	running 2 tests
	is 2 + 2 = 5?
	is 2 + 2 = 4?
	test tests::passing_test ... ok
	test tests::failing_test ... FAILED

To avoid that the test output and test results are interleaved, restricting the
test execution to one thread serializes the test execution and hence the
output:

	$ cargo test -- --nocapture --test-threads=1

	running 2 tests
	test tests::failing_test ... is 2 + 2 = 5?
	FAILED
	test tests::passing_test ... is 2 + 2 = 4?
	ok

### Test Selection

A subset of the available test cases can be run by indicating an expression to
be matched by the names of the test functions to be executed. Given this
module:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn add_positive_to_positive() {
        assert_eq!(7, add(3, 4))
    }

    #[test]
    fn add_negative_to_positive() {
        assert_eq!(2, add(-4, 6))
    }

    #[test]
    fn add_negative_to_negative() {
        assert_eq!(-9, add(-2, -7))
    }
}
```

The expression `add` will match all three test functions:

	$ cargo test add

	running 3 tests
	test tests::add_negative_to_negative ... ok
	test tests::add_positive_to_positive ... ok
	test tests::add_negative_to_positive ... ok

	test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Whereas the expression `add_neg` will only match two of the three test cases,
filtering out the third one:

	$ cargo test add_neg

	running 2 tests
	test tests::add_negative_to_negative ... ok
	test tests::add_negative_to_positive ... ok

	test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out

### Ignoring Tests

A test case annotated with the `#[ignore]` attribute won't be run by default:

```rust
#[test]
#[ignore]
fn add_big_to_big() {
	assert_eq!(1000000, add(999999, 1))
}
```
	$ cargo test

	running 4 tests
	test tests::add_big_to_big ... ignored
	test tests::add_negative_to_negative ... ok
	test tests::add_positive_to_positive ... ok
	test tests::add_negative_to_positive ... ok

	test result: ok. 3 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out

Ignored tests can be run separately by setting the `--ignored` flag:

	$ cargo test -- --ignored

	running 1 test
	test tests::add_big_to_big ... ok

	test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out

## Test Organization

### Unit Tests

Unit tests in Rust are written in a sub-module called `tests` and only test
their super-module, not code of any other modules. The privacy rules of Rust
allow a sub-module to call the private functions of its super-module, and
hence, in contrast to many other test frameworks and programming languages,
unit tests in Rust can also cover the private functions of a module. For unit
tests, the `test` module must be annotated with `#[cfg(test)]`.

This module contains public functions for addition and multiplication, and a
`tests` sub-module testing both the public and private functions
(`calc/src/lib.rs`):

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    add_multiple_times(a, b)
}

fn add_multiple_times(a: i32, n: i32) -> i32 {
    let mut product = 0;
    for _i in 0..n {
        product += a;
    }
    product
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
    }

    #[test]
    fn test_add_multuple_times() {
        assert_eq!(add_multiple_times(3, 2), 6);
    }
}
```

The multiplication is implemented as a repeated addition, and the unit test is
able to cover both the public and private API.

### Integration Tests

Integration tests in Rust only cover the public interface of the module to be
tested. The test code is organized in a folder called `tests`, located next to
the `src` folder. Since the integration test is neither part of the module to
be tested, nor a sub-module thereof, the module to be tested must be integrated
as an external crate (`calc/tests/calc_test.rs`):

```rust
extern crate calc;

#[test]
fn test_add() {
    assert_eq!(calc::add(2, 3), 5);
}

#[test]
fn test_multiply() {
    assert_eq!(calc::multiply(3, 4), 12);
}
```

The `#[cfg(test)]` annotation is _not_ needed here. The integration test only
covers the public API and thus simulates the usage of the module by another
project.

Integration tests can be executed separately from unit tests by indicating the
test files (without `.rs` suffix) to be executed:

	$ cargo test --test calc_test

Each file in the `tests` folder is compiled as its own separate crate. Common
functionality can be extracted and put into a module, for example called
`common`.

This test sub-module sets up a test case for the addition test
(`tests/common.rs`):

```rust
pub struct AdditionTest {
    pub a: i32,
    pub b: i32,
    pub expected: i32,
}

pub fn get_add_test_case() -> AdditionTest {
    AdditionTest {
        a: 3,
        b: 5,
        expected: 8,
    }
}
```

It can be used like this (`tests/calc_test.rs`):

```rust
extern crate calc;

mod common;

#[test]
fn test_add() {
    let test_case = common::get_add_test_case();
    assert_eq!(calc::add(test_case.a, test_case.b), test_case.expected);
}
```

If the module is defined in `tests/common.rs`, the module will be treated as an
additional integration test (just without any test methods), but not so if the
module is defined in `tests/common/mod.rs`.

### Binary Crates

The functions in the `src/main.rs` file (of a binary crate) cannot be covered
by integration tests. However, functions living in `src/lib.rs` can be both
used by the code in `src/main.rs` and covered by unit and integration tests.
Therefore, the code in `src/main.rs` should be kept small, moving as much of it
to modules at possible, so that it can be automatically tested.

For example, the `main` function of the `calculator` binary crate only calls
the `add` function from `src/lib.rs`:

```rust
extern crate calculator;

fn main() {
    println!("3 + 2 is {}", calculator::add(3, 2));
}
```

The file `src/lib.rs` contains both the `add` function and a test case for it:

```rust
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::add;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

The `add` function can also be tested using an integration test
(`test/integration_tests.rs`):

```rust
extern crate calculator;

#[test]
fn test_add() {
    assert_eq!(calculator::add(2, 3), 5);
}
```
