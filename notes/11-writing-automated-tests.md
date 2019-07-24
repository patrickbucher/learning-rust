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

TODO: p. 215
