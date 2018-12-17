# Common Programming Concepts

Rust offers the common features of structured programming languages, but
implements some in a special way.

## Variables and Constants

Variables are immutable by default:

```rust
let x = 3;
x = 5; // error: cannot assign twice to immutable variable `x`
```

Only the values of variables declared as mutable can be changed:

```rust
let mut x = 3;
x = 5; // OK
```

Variables can be redeclared, even their type can be changed:

```rust
let x = 3;
let x = x * x;
let x = x + 1;
println!("{}", x); // 10

let x = "10110";
let x = x.len();
println!("{}", x); // 5
```

In the examples above, five immutable variables called `x` have been declared;
no variable was ever changed. This technique is called _shadowing_: the second
`x` shadows the first `x`, the third `x` shadows the second `x`, etc.

Constants are a lot like immutable variables, but:

1. cannot be declared as mutable
2. are declared using the `const` keyword (as apposed to `let`)
3. can be declared in any scope
4. can only be assigned to expressions known at compile time
5. use the `ALL_UPPERCASE` naming convention
6. require a type annotation

```rust
const SPEED_OF_LIGHT: u32 = 299_792_458; // in meters per second (vacuum)
```

The `_` (underscore) is for optical groupings of three and has no special
meaning.

## Data Types

Rust is statically typed. The types of variables must be known at compile time.
In many cases, the type can be inferred from the context, in other cases, the
type must be annotated:

```rust
let foo = "test"; // string inferred
let bar = 10_000; // integer inferred

let qux: u32 = "42".parse().expect("can't parse to u32");
```

Without the annotation for `qux`, the compiler wouldn't know to which type
`parse()` has to convert the given string.

### Primitive Types

Rust offers four basic types of scalar (single values): integers, floating
point numbers, booleans and characters.

#### Integers

Integers exist as signed and unsigned variants:

| Size         | Signed                  | Unsigned   |
|--------------|-------------------------|------------|
| 8-bit        | `i8`                    | `u8`       |
| 16-bit       | `i16`                   | `u16`      |
| 32-bit       | `i32`                   | `u32`      |
| 64-bit       | `i64`                   | `u64`      |
| Architecture | `isize`                 | `usize`    |
| Range        | `-(2^[n-1])..2^[n-1]-1` | `0..2^n-1` |

In general, `i32` works fastest, even on 64-bit platforms.

The type names can be used as suffixes in literals:

```rust
let a: u8 = 255;
let a = 255u8;

let b: i8 = -128;
let b = -128i8;
```

Integer literals can be written in binary, octal, decimal (default) and hexadecimal notation:

```rust
let base2 = 0b01100100; // binary: prefix 0b
let base8 = 0755; // octal: prefix 0
let base10 = 1234567890; // decimal: no prefix
let base16: u32 = 0xdeadbeef; // hexadecimal: prefix 0x
```

The hexadecimal number needs a type annotation, because the signed 32-bit
integer (`i32`) inferred is too small for it.

There is a special byte prefix to convert ASCII characters into numbers:

```rust
let ascii_capital_a = b'A'; // 65
```

Arithmetic operators can be applied both to variables and literals:

```rust
let sum = 3 + 5; // 8
let difference = sum - 5 // 3
let product = difference * sum; // 24
let quotient = product / 2; // 12
let remainder = quotient * 5; // 2
```

#### Floating Point Numbers

Rust supports floating point numbers according to the IEEE-754 standard with
single precision (`f32`) and double precision (`f64`). A lot of the integer
notations and conventions can be used for floating point numbers, too:

```rust
let a = 13.2;
let b: f32 = 3.41;
let c = 5.324f64;
let d = 123_456.789;
```

#### Boolean

The `bool` type knows two values: `true` and `false`:

```rust
let right = true; // type inferred
let wrong: bool = false; // with type annotation
```

#### Character

Characters in Rust are UTF-8 encoded, and therefore not the same thing as a
single byte:

```rust
let latin_lower_c = 'c'; // 99 (requires one byte)
let cyrillic_upper_d = 'Д'; // 1044 (requires two bytes)
```

### Compound Types

Rust has two compound types: tuples and arrays.

#### Tuples

A tuple groups together values of (possibly) different types. A tuple's
elements can be accessed using dot-notation (using a zero-based index) or
through the means of destructuring:

```rust
let t: (i32, f64, u8) = (123, 4.56, 78);

// dot-notation
let a = t.0;
let b = t.1;
let c = t.2;

// destructuring
let (a, b, c) = t;
```

#### Arrays

Rust's arrays have a fixed size, so elements can be neither added nor removed,
but replaced if the array is declared as mutable. Unlike tupels, all elements
of an array must be of the same type.

```rust
let mut a = [1, 2, 3, 4, 5];
a[0] = 5;
a[4] = 1;
println!("[{},{},{},{},{}]", a[0], a[1], a[2], a[3], a[4]); // [5,2,3,4,1]
```

The usage of out-of-bounds indices either causes a runtime panic (if the index
value is computed at runtime) or doesn't even compile (if the index value can
be computed at compile time).

## Functions

Function names should follow the `snake_case` convention, i.e. all letters are
in lowercase, and the words are separated by an underscore `_`.

The order of the function's declarations doesn't matter; function calls are
possible forwards and backwards.

The parameter and return types of a function are not inferred and must be
declared explicitly. The parameter's types are annotated as for variables; the
return type is indicated after an arrow (`->`):

```rust
fn sum_up(a: i32, b: i32) -> i32 {
    a + b
}
```

A function can either end in an expression (as above), which is used as the
function's return value, or an expression can be returned explicitly using the
`return` statement:

```rust
fn sum_up(a: i32, b: i32) -> i32 {
    return a + b;
}
```

Any block can return a value:

```rust
let y = {
    let x = 3;
    x + 5 // expression: no semicolon
}; // semicolon, ending the let statement
```

Because statements do _not_ return values, this code doesn't compile:

```rust
let c = (let b = (let c = 1)); // error: expected expression
```

## Comments

Single-line comments start with `//` and end at the line's end.

Multi-line comments start with `/*` and end with `*/`.

## Control Flow

### Conditional Execution

Only `bool` expressions are allowed for `if` conditions:

```rust
if x < 4 {
    println!("low");
} else if x < 7 {
    println!("medium");
} else {
    println!("high");
}
```

`if` is an expression, not a statement, and therefore can return a value:

```rust
let max = if a > b {
    a
} else {
    b
};
```

It's important that the expressions of both arms are of the same data type!

### Loops

`loop` runs infinitely, unless ended with `break`:

```rust
let numbers = [1, 2, 3, 4, 5];
let mut i = 0;
loop {
    println!("{}", numbers[i]);
    if i >= 4 {
        break;
    } else {
        i += 1;
    }
}
```

`while` checks a condition on every iteration before the its block is executed:

```rust
let numbers = [1, 2, 3, 4, 5];
while i < 5 {
    println!("{}", numbers[i]);
    i += 1;
}
```

`for` iterates over the items of a collection, e.g. an array:

```rust
let numbers = [1, 2, 3, 4, 5];
for i in numbers.iter() {
    println!("{}", i);
}
```

The `for` loop is by far the most commonly used in Rust.

`continue` leaves the loop's block and moved forward to the next iteration:

```rust
let numbers = [1, 2, 3, 4, 5];
for i in numbers.iter() {
    if i % 2 == 0 {
        continue; // skip even numbers
    }
    println!("{}", i);
}
```
