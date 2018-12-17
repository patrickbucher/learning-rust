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
|--------------|-------------------------|------------|
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

## Functions

## Comments

## Control Flow
