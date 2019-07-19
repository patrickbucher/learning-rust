# Traits, Generics and Lifetimes

## Traits

Traits are Rust's way of declaring common functionality (a set of method
signatures) that can be implemented by different types.

A trait is defined using the `trait` keyword, following a list of method
signatures:

```rust
pub trait Summary {
	fn summarize(&self) -> String;
}
```

The trait is public, so that it can be implemented in other crates as well.

Given the structs `NewsArticle` and `Tweet`:

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}
```

The `Summary` trait can be implemented for them using the `for` keyword:

```rust
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

The `summarize` method can then be used like this:

```rust
fn main() {
    let n = NewsArticle {
        headline: "Sack of Rice Fell Down".to_string(),
        location: "China".to_string(),
        author: "Russel F. Important".to_string(),
        content: "Well, the headline says it all...".to_string(),
    };
    let t = Tweet {
        username: "@russelfimportant".to_string(),
        content: "Sack of Rice fell down in China.".to_string(),
        reply: false,
        retweet: false,
    };
    println!("NewsArticle: {}", n.summarize());
    println!("Twitter: {}", t.summarize());
}
```

Output:

	NewsArticle: Sack of Rice Fell Down, by Russel F. Important (China)
	Twitter: @russelfimportant: Sack of Rice fell down in China.

A trait can only be implemented if either the trait or the type is local to the
crate. External traits cannot be implemented on external types.

### Default Implementations

A trait can define a default behaviour for a method, i.e. providing a method
implementation that can but does not has to be overwritten. If the default
implementation is not to be overwritten, the trait can be implemented with an
empty `impl` block:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct BlogPost {
    pub title: String,
    pub url: String,
    pub author: String,
}

impl Summary for BlogPost {}
```

Default implementations can call other methods of the same trait, even those
without a default implementation. It is not possible to call the default
implementation from an overriding implementation.


## Generics

Instead of writing the same data structure or function over and over for
different types (say, `i32` and `f32` for a mathematics library that supports
both integer and floating point arithmetic), types can be parametrized using
_generics_.

### Generic Structs

The two structs `PointDiscrete` and `PointContinuous` define the same fields
(`x` and `y`), but use different type for those: integers and floats.

```rust
struct PointDiscrete {
    x: i32,
    y: i32,
}

struct PointContinuous {
    x: f32,
    y: f32,
}
```

Those two types can almost be used alike, but for the type system, there
something fundamentally different.

```rust
fn main() {
    let a = PointDiscrete { x: 12, y: 7 };
    let b = PointContinuous { x: 3.75, y: 2.12 };
    println!("a=({}, {}), b=({}, {})", a.x, a.y, b.x, b.y);
}
```

Introducing a generic type parameter `T` not only helps merging the two struct
definitions to one, but also allows using other types, such as `f64` or `u16`:

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let a = Point { x: 12, y: 7 };
    let b = Point { x: 3.75, y: 2.12 };
    println!("a=({}, {}), b=({}, {})", a.x, a.y, b.x, b.y);
}
```

The actual types of point `a` and `b` are inferred. Since there's only a single
type parameter defined, and `T` is the same for the field `x` and `y`, the
following code won't compile:

```rust
let c = Point { x: 10, y: 3.14 };
println!("c=({}, {})", c.x, c.y);
```

The compiler infers some integer type for `x`, so `y` must also be an integer:

error[E0308]: mismatched types
  --> src/main.rs:11:31
   |
11 |     let c = Point { x: 10, y: 3.14 };
   |                               ^^^^ expected integer, found floating-point number
   |
   = note: expected type `{integer}`
              found type `{float}`

If two type parameters, `T` und `U`, are used for the struct definitions, the
above code compiles, no matter if `T` and `U` are the same types or different:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let a = Point { x: 12, y: 7 };
    let b = Point { x: 3.75, y: 2.12 };
    println!("a=({}, {}), b=({}, {})", a.x, a.y, b.x, b.y);

    let c = Point { x: 10, y: 3.14 };
    println!("c=({}, {})", c.x, c.y);
}
```

However, using different types for `x` and `y` could make it harder to use
common operations on those fields: The less the compiler knows about a type,
the fewer operations can be performed on fields of that type. Therefore generic
type parameters should only be used if it clearly serves the use case at hand,
and not for flexibility for it's own sake.

### Generic Enums

Rust's two most common enums, `Option<T>` and `Result<T, E>` use generic type
parameters:

```rust
enum Option<T> {
	Some(T),
	None,
}
```

`Option` only has a single type parameter, `T`. The `None` case does not need a
type, because it is used to signify the absence of a value, and hence the
absence of a type.

```rust
enum Result<T, E> {
	Ok(T),
	Err(E),
}
```

`Result`, however, is parametrized with two types, `T` and `E`; the `E`
parameter is used for some error type, whereas the `T` parameter specifies the
type for the result of a successful operation, which usually is not an error
message, even though this enum definition does not forbid using it that way.

### Generic Functions

The two functions `largest_i` and `largest_f` take a list of integers (`i32`)
or floats (`f32`), respectively, find the biggest item in the list and return
it:

```rust
fn largest_i(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn largest_f(list: &[f32]) -> f32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}

fn main() {
    let integers = vec![53, 12, 76, 19, 44, 98, 72];
    let floats = vec![1.45, 93.9, 64.3, 91.3, 45.1];

    println!("largest of {:?}: {}", integers, largest_i(&integers));
    println!("largest of {:?}: {}", floats, largest_f(&floats));
}
```

Even though the method only differs in the signature and body (the
implementation) is exactly the same twice, the whole code has been duplicated.

Using generics, type information can be parametrized to avoid such duplication.
This implementation replaces the specific types `i32` and `f32` with a generic
type parameter `T` (a common convention for "type"):

```rust
fn largest<T>(list: &[T]) -> T { // T instead of i32/f32
	// implementation unchanged
}
```

Unfortunately, this method does not compile:

	error[E0369]: binary operation `>` cannot be applied to type `T`
	  --> src/main.rs:29:17
	   |
	29 |         if item > largest {
	   |            ---- ^ ------- T
	   |            |
	   |            T
	   |
	   = note: `T` might need a bound for `std::cmp::PartialOrd`

Comparisons with the `>` operator do not work for _all_ types, but only for
types that implement the trait `std::cmp::PartialOrd` (as the compiler
suggests).

### Trait Bounds

The types that can be used for a generic type parameter can be constrained
using a _trait bound_. The type parameter (`<T>`) is supplied with a trait name
(`<T: Trait>`), so that the function is only appliable to types satisfying the
given trait. It's also possible to constrain a type using multiple traits by
separating those traits with a `+`, such as `<T: Trait1 + Trait2>`.

The `largest` function from above can be made to work by constraining the type
parameter `T` to the traits `PartialOrd` (for the `>` comparison) and `Copy`
(so that the items from the list are copied and not moved out of the list):

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}
```

An alternate syntax using the `where` clause helps keeping the function
signature short:

```rust
fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest
}
```

The difference is more striking if multiple type parameters are involved:

```rust
fn set<K: Display + Copy, V: PartialOrd + Copy>(key: K, value: V) {
	// ...
}
```

Compared to:

```rust
fn set<K, V>(key: K, value: V)
where
	K: Display + Copy,
	V: PartialOrd + Copy,
{
	// ...
}
```

### Generic Methods

To implement a method for all possible types on a struct with a type parameter,
the type parameter must also be declared for the `impl` block:

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &T {
        &self.y
    }
}
```

It is also possible to implement certain methods only for specific types, say,
floating point numbers, which support additional mathematical operations (e.g.
square roots) compared to integers:

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    fn distance_from(&self, other: &Point<f32>) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 3.75, y: 2.12 };
    let q = Point { x: 1.5, y: 8.95 };

    println!("distance p to (0, 0): {}", p.distance_from_origin());
    println!("distance q to (0, 0): {}", q.distance_from_origin());
    println!("distance p to q: {}", p.distance_from(&q));
    println!("distance q to p: {}", q.distance_from(&p));
}
```

The methods can be called on any `Point` using `f32` values. However, if a
point uses integer values, the methods above are not available:

```rust
fn main() {
    let a = Point { x: 1, y: 2 };
    println!("distance a to (0, 0): {}", a.distance_from_origin())
}
```

Output:

	error[E0599]: no method named `distance_from_origin` found for type `Point<{integer}>` in the current scope
	  --> src/main.rs:17:44
	   |
	1  | struct Point<T> {
	   | --------------- method `distance_from_origin` not found for this
	...
	17 |     println!("distance a to (0, 0): {}", a.distance_from_origin())
	   |                                            ^^^^^^^^^^^^^^^^^^^^

The type parameters of a method parameter are not restricted to the type
parameters of the `impl` block. Starting from this struct definition with
different types for `x` and `y`:

```rust
struct Point<T, U> {
    x: T,
    y: U,
}
```

A method can be implemented that works on a `Point` with certain type
parameters, while accepting `Point` parameters that use (possibly) different
type parameters:

```rust
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,  // T
            y: other.y, // W
        }
    }
}

fn main() {
    let a = Point { x: 1, y: 2.5 };
    let b = Point { x: 3.99, y: 8 };
    let p = a.mixup(b);
    println!("p=({},{})", p.x, p.y); // Output: p=(1,8)
}
```

The type parameters `T` and `U` are the types of the callee (`self`), whereas
`V` and `W` are the types of the method parameter (`other`). The method `mixup`
produces a new point with mixed-up type parameters. For this purpose, type
parameters from both the `impl` block and the method signature can be used.

### Conditionally Implement Methods

Methods can be implemented conditionally for types that satisfy specific
traits. The `cmp_display` method can only be invoked on a `Pair<T>` whose `T`
implements both the `Display` and the `PartialOrd` trait:

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            // PartialOrd
            println!("The largest member is x={}", self.x); // Display
        } else {
            println!("The largest member is y={}", self.y); // Display
        }
    }
}

fn main() {
    let p = Pair { x: 3, y: 7 };
    p.cmp_display();
}
```

Since integer types satisfy both traits, the code above compiles, and the
`cmp_display` method can be called on the `Pair p`.

It is also possible to implement a trait for any type that implements another
trait. Let's say that the method `identify` should implemented for all the
types that satisfy the `Display` trait:


```rust
use std::fmt::Display;

trait Subject {
    fn identify(&self);
}

impl<T: Display> Subject for T {
    fn identify(&self) {
        println!("I am {}.", self);
    }
}

fn main() {
    let pi = 3.14;
    pi.identify(); // I am 3.14.
}
```

The `identify()` method is now available for all other types satisfying
`Display`, without providing any futher implementations. Such implementations
are called _blanket implementations_. They are documented in the "Implementors"
section to each trait.

### Compilation and Performance

The Rust compiler turns generic code into code with specific types upon
compilation (_monomorphization). If a struct `Point<T>` is used with the
specific types `i32` and `f32`, the compiler will fill in the types and produce
two implementations: `Point_i32` and `Point_f32`. For any usage of some
`Point<T>`, the type parameter will be inferred, and the code will be compiled
using the specific types.

This comes at some cost: Not only is the compilation process more complicated
and therefore slower, but also is the resulting code gets bigger. However,
handling generics at compile time ensures that there are no runtime costs; and
because a programm is usually compiled once and run many times, the additional
compilation time can be seen as a investment rather than a cost.

## Lifetimes

TODO:

- example that works without lifetimes (one parameter)
- example that fails without lifetimes (two parameters)
- explain the borrow checker and the motivation behind it
- explain this using the _lifetime elision_ rules (p. 196)
- introduce lifetime annotation syntax (p. 190)
- fix example with two parameters from above using lifetime annotations
- static lifetimes (p. 198)
