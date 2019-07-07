# Enumerations and Pattern Matching

Enumerations (short: _enums_) are types defined by enumerating its possible
values and encode meaning along with data. The enum's different values are
called its _variants_.

A enum is defined using the `enum` keyword and a list of variants within curly
braces:

```rust
enum ColorType {
    RGBA,
    CMYK,
}
```

The variants of an enum belong to the same type. They are namespaced under the
enum's name and need to be qualified in order to be used:

```rust
let rgba = ColorType::RGBA;
let cmyk = ColorType::CMYK;
```

A function can accept values of either variant by annotating the enum's name as
the expected type:

```rust
fn list_colors(color_type: ColorType) {
    // ...
}

fn main() {
    list_colors(ColorType::RGBA;);
    list_colors(ColorType::CMYK);
}
```

Variants can hold their own data. The amount, structure and type of the
associated data can vary between different variants. It is possible to use
single scalars, tuples, anonymous structs and even other enums:

```rust
#[derive(Debug)]
enum ColorType {
    Unspecified,           // no data
    Named(String),         // single value
    RGBA(u8, u8, u8, f32), // tuple
    CMYK {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    }, // anonymous struct
}
```

Even though the variants are made up of different types, they still belong to
the same enum type.

Enums can have methods attached to them in `impl` blocks:

```rust
impl ColorType {
    fn print(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let none = ColorType::Unspecified;
    let fuchsia = ColorType::Named(String::from("fuchsia"));
    let red = ColorType::RGBA(255, 0, 0, 0.5);
    let yellow = ColorType::CMYK {
        cyan: 0,
        magenta: 0,
        yellow: 100,
        black: 0,
    };

    none.print();
    fuchsia.print();
    red.print();
    yellow.print();
}
```

Output:

    Unspecified
    Named("fuchsia")
    RGBA(255, 0, 0, 0.5)
    CMYK { cyan: 0, magenta: 0, yellow: 100, black: 0 }

## The `Option<T>` Enumeration

Unlike many other programming language, Rust has no `null` (or `nil`)
references. The absence (and presence) of a value is instead expressed by the
`Option<T>`; an enum with two variants and a type parameter `T`:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

The variant `Some` holds a value of type `T`. The variant `None` is used to
signify the absence of a value and therefore does not hold any. The enum
`Option<T>` and its variants `Some(T)` and `None` are included in the prelude
and hence are not required to be made available first. They can be used without
qualification:

```rust
let a_number = Some(42);
let a_word = Some(String::from("whatever"));
let none: Option<i32> = None;
```

The type can be inferred in case of `Some` but must be annotated for `None`,
for there is no value to infer from.

A instance of `Option<T>` can not be treated like an instance of `T`:

```rust
let a: i32 = 3;
let b: Option<i32> = Some(4);
let c: Option<i32> = None;

let x = a + b; // error
let y = a + c; // error
```

The variable `a` is of type `i32`, while `b` and `c` are of type `Option<i32>`.
Trying to handle them alike causes a compilation error. This is exactly the
point, because adding a _only possibly_ available value to a existing value
cannot be guaranteed to work. The compiler makes sure that a value is always
there when needed.

`Option<T>` must be converted to `T` before it can be used. The enum offers
methods for that purpose, but the general approach is to use pattern matching.

## Pattern Matching

The `match` control flow operator allows to compare a value against a series of
patterns and executes a branch of code for the matching pattern.

In the context of enums, `match` allows to execute different code for a value
according to its variant. Every variant has an expression, statement or block
assigned with the `=>` operator, called the variant's _arm_:

```rust
enum Animal {
    Blobfish,
    Human,
    Fox,
    Octopus,
    Centipede,
}

fn number_of_legs(animal: Animal) -> u8 {
    match animal {
        Animal::Blobfish => 0,
        Animal::Human => {
            println!("not counting arms");
            2
        },
        Animal::Fox => 4,
        Animal::Octopus => {
            println!("arms or legs? legs!");
            return 8;
        },
        Animal::Centipede => 100,
    }
}

fn main() {
    let johnny = Animal::Human;
    let legs = number_of_legs(johnny);
    println!("{} legs", legs);
}
```

If one or many variants were missing, the program would not compile:

```rust
fn number_of_legs(animal: Animal) -> u8 {
    match animal {
        Animal::Blobfish => 0,
        Animal::Fox => 4,
        Animal::Centipede => 100,
    }
}
```

    match animal {
          ^^^^^^ patterns `Human` and `Octopus` not covered

Matches are said to be _exhaustive_; the compiler makes sure that all the
possibilities are handled. If one or many variants are _not_ of interest, the
`_` pattern, which matches to anything, can be used for the last arm. (If it
were not the last arm, the subsequent patterns could not possibly match,
because `_` already matched everything.)

```rust
fn number_of_legs(animal: Animal) -> u8 {
    match animal {
        Animal::Blobfish => 0,
        Animal::Fox => 4,
        Animal::Centipede => 100,
        _ => 0,
    }
}
```

If no code were to be executed for the default case, the unit value `()` can be
used:

```rust
match animal {
    Animal::Blobfish => println!("0"),
    Animal::Fox => println!("4"),
    Animal::Centipede => println!("100"),
    _ => (),
}
```

This only works if `match` is used as a statement as opposed to an expression;
the latter always needs to yield a value.

If only a single variant is of interest, the `if let` construct can be used to
make the code more concise. Consider this `match` expression:

```rust
let val = Some(42);
match val {
    Some(42) => println!("correct"),
    _ => (),
}
```

Which can be rewritten using the `if let [pattern] = [value]` pattern as
follows:

```rust
let val = Some(42);
if let Some(42) = val {
    println!("correct");
}
```

The resulting code is shorter, but the compiler no longer checks if all
variants are being handled.

One common use case for pattern matching is to extract a value of an enum's
variant. Consider this enum with a enhanced `Centipede` definition, of which
the value needs to be extracted:

```rust
enum Animal {
    Blobfish,
    Human,
    Fox,
    Octopus,
    Centipede(String),
}

fn number_of_legs(animal: Animal) -> u8 {
    match animal {
        Animal::Blobfish => 0,
        Animal::Human => 2,
        Animal::Fox => 4,
        Animal::Octopus => 8,
        Animal::Centipede(kind) => {
            if kind == "Stone Centipede" {
                30
            } else if kind == "Giant Readhead Centipede" {
                42
            } else {
                100
            }
        }
    }
}

fn main() {
    let centi = Animal::Centipede(String::from("Stone Centipede"));
    let legs = number_of_legs(centi);
    println!("{} legs", legs);
}
```

The `Animal::Centipede(kind)` pattern is said to _bind to a value_. This
technique is also used to unpack the `T` instance of a `Option<T>`. Consider
this safe division function, which returns the resulting quotient wrapped in a
`Option<f32>`—or `None`, if the division is undefined when the divisor is zero:

```rust
fn divide(dividend: i32, divisor: i32) -> Option<f32> {
    if divisor == 0 {
        return None;
    }
    Some(dividend as f32 / divisor as f32)
}
```

The result of the `divide()` function is handled with pattern matching,
ensuring that always a value present by mapping `None` to `0`:

```rust
let a = 10;
let b = 3;
let c = 0;

let x = match divide(a, b) {
    Some(q) => q,
    None => 0.0,
};
let y = match divide(a, c) {
    Some(q) => q,
    None => 0.0,
};

println!("x={}, y={}", x, y); // x=3.3333333, y=0
```
