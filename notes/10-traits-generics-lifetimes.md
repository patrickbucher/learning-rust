# Traits, Generics and Lifetimes

TODO: common introductory words

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

TODO: p. 182 Trait Bounds

TODO: p. 185 conditionally implement methods

TODO: p. 173 generics for structs

TODO: p. 174 generics for enums

TODO: p. 175 generics for methods

TODO: p. 177 note on performance

## Lifetimes

TODO: p. 187 ff.
