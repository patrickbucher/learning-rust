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

TODO: p. 181
