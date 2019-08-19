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

### Fixing the Cache Using a Map

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

## Iterators

An iterator allows to process its underlying items one by one. The iterator
takes care of the logic of iterating over the items, and determines when the
sequence has reached its end.

An iterator can be retrieved by calling the `iter` method on a collection:

```rust
let v = vec![1, 2, 3];
let i = v.iter();
```

The iterator's elements then can be iterated over using a `for`/`in` loop:

```rust
for e in i {
	println!("{}", e);
}
```

### The `Iterator` Trait and the `next` Method

An iterator must implement the `Iterator` trait, which is defined as follows:

```rust
pub trait Iterator {
	type Item;
	fn next(&mut self) -> Option<Self::Item>;
	// additional methods
}
```

The declaration `type Item` and the `Option`s type `Self::Item` define a
_associated type_. Implementing an iterator also requires defining an `Item`
type.

The `next` method is the only method that implementors need to define. It moves
on to the next item, and returns it wrapped up in a `Option`. The `self`
reference is mutable, because calling `next` modifies the iterator's underlying
state ‒ the current position in the iterator.

An iterator can not only be processed using a `for`/`in` loop, but also
directly using the `next` method. When doing so, the iterator reference has to
be mutable explicitly. (The `for`/`in` loop makes the iterator mutable in the
background.)

```rust
let v = vec![1, 2, 3];
let mut i = v.iter();
println!("{:?}", i.next());
println!("{:?}", i.next());
println!("{:?}", i.next());
println!("{:?}", i.next());
```

Output:

	Some(1)
	Some(2)
	Some(3)
	None

### Consuming Adaptors

Methods that call the `next` method use up the iterator, and therefore are
called _consuming adaptors`. The `sum` method is an example for this:

```rust
let v = vec![1, 2, 3];
let i = v.iter();
let total: i32 = i.sum();
println!("{}", total); // 6
```

The iterator `i` must not be used after calling the `sum` method. A `value used
after move` compiler error would be the consequence of doing so nonetheless.

### Iterator Adaptors

Methods that turn an iterator into another iterator are called _iterator
adaptors_. Iterators are _lazu evaluated_: A consuming adaptor has to be
applied in order to get the result of calling an iterator adaptor. For example,
the `map` method does not really create a new iterator:

```rust
let v = vec![1, 2, 3];
v.iter().map(|x| x + 1);
```

Calling the `collect` method ‒ a _consuming adaptor_ ‒ will produce a new
collection with the mapped elements (here: added one to them):

```rust
let v = vec![1, 2, 3];
let plus_one: Vec<_> = v.iter().map(|x| x + 1).collect();
println!("{:?}", plus_one); // [2, 3, 4]
```

### Filtering

The `filter` method takes a closure returning a boolean that is called for each
of the iterator's items in turn. If the closure returns `true`, the item is
included in the newly produces iterator; otherwise not.

```rust
let v = vec![1, 2, 3, 4, 5];
let even: Vec<_> = v.into_iter().filter(|x| x % 2 == 0).collect();
println!("{:?}", even); // [2, 4]
```

Notice that here the `into_iter` method is called, which takes ownership of the
underlying collection. (Calling `iter_mut` would borrow the underlying elements
mutably.)

### Implementing an Iterator

To implement an own iterator, only the `next` method has to be implemented. A
counter that runs from `0` to an arbitrary `limit` is defined as follows:

```rust
struct Counter {
    count: u32,
    limit: u32,
}

impl Counter {
    fn new(limit: u32) -> Counter {
        Counter {
            count: 0,
            limit: limit,
        }
    }
}
```

The `iterator` trait can be implemented as follows:

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count <= self.limit {
            Some(self.count)
        } else {
            None
        }
    }
}
```

The elements type is defined as `u32` using a type association. The `next`
method increases its internal counter by one. If the counter is still within
the limit, it is returned wrapped in the `Some` variant as an `Option`.
Otherwise, the `None` variant is returned.

The counter can be used as follows:

```rust
let c = Counter::new(3);
for e in c {
	println!("{}", e);
}
```

This counter now offers different useful iterator methods by default:

```rust
let c1to5 = Counter::new(5); // [1, 2, 3, 4, 5]
let c1to7 = Counter::new(7); // [1, 2, 3, 4, 5, 6, 7]
let sum: u32 = c1to5
	.zip(c1to7.skip(2))      // [(1, 3), (2, 4), (3, 5), (4, 6), (5, 7)]
	.map(|(a, b)| a * b)	 // [3, 8, 15, 24, 35]
	.filter(|x| x % 3 == 0)  // [3, 15, 24]
	.sum();
println!("{}", sum);         // 42
```
