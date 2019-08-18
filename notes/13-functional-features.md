# Functional Language Features

Rust is heavily influenced by _functional programming_, which involves using
functions as values.

## Closures

A _closure_ is a function-like construct that can be stored in a variable.
Those are anonymous functions that are able to capture values from the
surrounding scope. One common use case of closures is to abstract behaviour.

### Closure Syntax

The following closure, bound to the name `add`, adds up two numbers and returns
the result:

```rust
fn main() {
    let a = 3;
    let b = 5;
    let add = |x, y| x + y;
    let c = add(a, b);
    println!("{}", c);
}
```

Expressed as a function, `add` could be implemented as follows:

```rust
fn main() {
    let a = 3;
    let b = 5;
    fn add(x: u32, y: u32) -> u32 {
        x + y
    };
    let c = add(a, b);
    println!("{}", c);
}
```

Thus, the syntactical differences between a function and a closure are:

1. A function is declared using the `fn` keyword (expression), whereas a
   closure is usually bound to a name using the `let` keyword (statement).
2. A function's parameter list is surrounded by parentheses; closures use a
   pair of pipes (`|`) to surround the parameter list.
3. A function needs explicit type annotations for both the parameters and the
   return values. For closures, which usually are only relevant in a very
   limited scope, the compiler is able to figure out those types.
4. A function body needs to be surrounded by curly braces. For a closure with a
   body consisting of only a single expression, the curly braces can be left
   away.

The following declarations demostrate those differences:

```rust
fn	add1   (x: u32) -> u32 { x + 1 }  // 1. fn keyword, parentheses
let add2 = |x: u32| -> u32 { x + 1 }; // 2. let keyword, pipes
let add3 = |x|             { x + 1 }; // 3. no type annotations
let add4 = |x|               x + 1  ; // 4. no curly braces
```

### Closure Semantics

Closures are useful to store some code to be executed later, possibly in a
different context. This program declares the four basic arithmetic operations
as closures and invokes them:

```rust
fn main() {
    let a = 16;
    let b = 4;

    let add = |x, y| x + y;
    let sub = |x, y| x - y;
    let mul = |x, y| x * y;
    let div = |x, y| x / y;

    println!("{}", add(a, b));
    println!("{}", sub(a, b));
    println!("{}", mul(a, b));
    println!("{}", div(a, b));
}
```

As opposed to a function, a closure can capture variables from its environment:

```rust
fn main() {
    let a = 7;
    let add = |x| a + x; // capture binding a

    let z = 3;
    println!("{}", add(z)); // 7 + 3 = 10
}
```

The compiler infers the types of a closure the first time it is used.
Therefore, the same closure cannot be used twice with different types:

```rust
fn main() {
    let take_and_give = |value| value;

    let s = take_and_give(String::from("hello"));
    let n = take_and_give(5);
}
```

For the first call of `take_and_give`, the type `String` is inferred for the
type of the `value` parameter. The second call, which uses an integer instead
of a String, is therefore illegal:

	error[E0308]: mismatched types
	 --> src/main.rs:5:27
	  |
	5 |     let n = take_and_give(5);
	  |                           ^
	  |                           |
	  |                           expected struct `std::string::String`, found integer
	  |                           help: try using a conversion method: `5.to_string()`
	  |
	  = note: expected type `std::string::String`
				 found type `{integer}`

### Use-Case: Memoization

For expansive calculations, caching the results of an operation can safe a lot
of execution time. This closure computes the `n`th fibonacci number
iteratively, because closures cannot call themselves:

```rust
fn main() {
    let fib = |n| {
        if n < 2 {
            1
        } else {
            let mut prev1 = 0;
            let mut prev2 = 1;
            for _i in 0..n {
                let tmp = prev1 + prev2;
                prev1 = prev2;
                prev2 = tmp;
            }
            prev2
        }
    };
    for i in 0..10 {
        println!("fib({})={}", i, fib(i));
    }
}
```

Even though the recursive approach would be much heavier than this iterative
implementation, caching the results in the program still illustrates the point.

First, a struct shall be defined, which holds both the closure and the
calculated value:

```rust
struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    operation: T,
    value: Option<u32>,
}
```

The struct `Cache` has a type parameter that is satisfied by any `Fn` mapping a
`u32` to another `u32` ‒ exactly what the `fib` closure does.

`Fn` is a trait provided by the standard library, which indicates that the
function or closure at hand borrows values from its environment immutably. (The
trait `FnMut` borrows values mutably; the trait `FnOnce` consumes values, and
therefore can only be called once.)

The `value` field is declared as an `Option`, indicating that a value might
already have been calculated ‒ or not.

The `operation` and `value` fields are _not_ public, for the client should not
access them directly, but through a constructor and an additional method:

```rust
mpl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(operation: T) -> Cache<T> {
        Cache {
            operation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.operation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
```

The `new` function (the constructor) creates a new `Cache` with the given
operation and no value. The `value` method checks if there is already a value,
and returns it, if so. If not, `operation` is executed, and its result stored
in the `value` field and returned to the caller.

The `Cache` can then be used as follows:

```rust
fn main() {
    let fib = |n| {
        println!("called fib closure");
        if n < 2 {
            1
        } else {
            let mut prev1 = 0;
            let mut prev2 = 1;
            for _i in 0..n {
                let tmp = prev1 + prev2;
                prev1 = prev2;
                prev2 = tmp;
            }
            prev2
        }
    };
    let mut cache = Cache::new(fib);
    println!("{}", cache.value(9));
    println!("{}", cache.value(9));
}
```

The `fib` closure was slightly modified to print a message when it is invoked.
The 9th fibonacci number is requested two times, but the program only prints
that additional message once, because the result of the second request was
retrieved from the cache, and not calculated:

	$ cargo run
	called fib closure
	55
	55

This cache only works if it is used with the same parameter repeatedly. A
useful implementation would not only store one result, but map the input
parameter to cached results:

```rust
use std::collections::HashMap;

struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    operation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(operation: T) -> Cache<T> {
        Cache {
            operation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.operation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}
```

The following client code now works as expected:

```rust
let mut cache = Cache::new(fib);
println!("{}", cache.value(9));
println!("{}", cache.value(10));
println!("{}", cache.value(9));
println!("{}", cache.value(10));
```

The `fib` closure is only called for the first time a parameter is used:

	$ cargo run
	called fib closure
	55
	called fib closure
	89
	55
	89
