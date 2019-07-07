# Ownership

Rust neither has garbage collection nor requires manual memory management. It
uses a third approach: Memory is managed through a system of rules enforced by
the compiler―the ownership model. The rules are:

1. Each value has a variable that's called its _owner_.
2. There  can be only one owner at a time.
3. When the owner goes out of scope, the value is dropped.

Rust calls the `drop()` function for every variable at the end of its scope to
free the memory. The owner variable will be no longer valid from this point.

## Stack and Heap

Variables of primitive types (integers, floats, booleans, characters and tuples
solely consisting of primitive types) are stored entirely on the stack. If a
variable is assigned to a variable of a primitive type, the whole stack content
is copied:

```rust
let a = 3;
let b = a; // the value of a is copied
```

Types with sizes unknown at compile time, like strings, are stored on the heap:

```rust
let s1 = String::from("hello");
let s2 = s1; // the value of s1 is NOT copied
```

Copying the content of `s1` would be expensive in terms of runtime performance.
Therefore, only the pointer to the string's heap memory location is copied,
i.e. the variable's stack content is (just like for primitives), but here the
stack content is a pointer to the heap. (Note: This is _not_ what actually
happens; see below!)

The content of a heap variable can be created using the `clone()` method:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

## Moving

The different handling of stack and heap objects has consequences in regard to
memory management. The automatic `drop()` on primitive variables at the end of
a scope is unproblematic:

```rust
// before: a and b out of scope
{
    let a = 3;
    let b = a;
}
// after: a and b out of scope
```

The value of `a` was copied, and only the stack memory of `a` and `b` is freed.
The behaviour is different for objects on the heap:

```rust
// before: s1 and s2 out of scope
{
    let s1 = String::from("hello");
    let s2 = s1;
    // call drop() on s1 and s2
}
// after: s1 and s2 out of scope
```

Freeing the same memory twice corrupts the memory, could cause a program to
crash and might cause a security vulnerability. To prevent such problems, the
assignment `s2 = s1` in the program above does _not_ create a _shallow copy_
(and also not a _deep copy_, as already mentioned). The value is instead
_moved_ from `s1` to `s2`, and `s2` becomes the new owner. The variable `s1`
cannot be used any longer from that point:

```rust
let s1 = String::from("hello");
let s2 = s1; // value of s1 moves to s2
println!("s1={}", s1); // invalid, value moved to s2
println!("s2={}", s2); // valid, s2 is the new owner
```

### Move on Function Call

A function call behaves a lot like an assignment in terms of ownership of the
parameters. A function expecting a `String` will take ownership of that value,
but a integer value will just be copied:

```rust
fn move_or_copy(str: String, nbr: i32) {
    println!("str={}, nbr={}", str, nbr);
}

fn main() {
    let s = String::from("abc");
    let i = 42;
    move_or_copy(s, i); // move s, copy i
    println!("s={}", s); // invalid: s was moved
    println!("i={}", i); // valid: i was copied
}
```

The ownership of `s` can be moved back by returning it from the function:

```rust
fn move_or_copy(str: String, nbr: i32) -> String {
    println!("str={}, nbr={}", str, nbr);
    str // return
}

fn main() {
    let s = String::from("abc");
    let i = 42;
    let s2 = move_or_copy(s, i); // take the ownership of the object back
    println!("s2={}", s2); // valid now!
    println!("i={}", i);
}
```

## Borrowing with References

Obtaining the ownership of a variable and giving it back is tedious. A variable
can be _borrowed_ instead by using a reference. A reference is denoted by an
ampersand in front of the type (`&String`) and value (`&s`):

```rust
fn borrow(str: &String) { // &String: reference to String
    println!("str={}", str);
}

fn main() {
    let s = String::from("abc");
    borrow(&s); // &s is a reference to s
    println!("s={}", s); // valid: s was only borrowed, not owned
}
```

The value of a reference can only be modified if the reference is mutable.
Mutable references are denoted by the token `&mut` in front of the type (`&mut
String`) and value (`&mut s`). Mutable references can only be acquired from
mutable variables:

```rust
fn manipulate(str: &mut String) { // &mut String: mutable reference to String
    str.push_str("...xyz"); // allows for manipulation
}

fn main() {
    let mut s = String::from("abc"); // mutable variable
    manipulate(&mut s); // mutable reference
    println!("s={}", s); // valid: s was only borrowed, not owned
}
```

### Data Races

Having multiple references to a memory object allows for _data races_: The
values is updated through one reference, and the other references are not aware
of that. Rust eliminates data races by enforcing a very strict set of rules:

> One can only have:
>
> 1) either one mutable reference
> 2) or multiple immutable references
>
> to the same value in the same scope.

One mutable reference is ok:

```rust
let mut s = String::from("hello");
let r1 = &mut s;
println!("{}", r1);
```

Two mutable references are _not_ ok:

```rust
let mut s = String::from("hello");
let r1 = &mut s;
let r2 = &mut s; // error, only one mutable reference allowed
println!("{}", r1);
println!("{}", r2);
```

However, two mutable references _in different scopes_ are ok:

```rust
let mut s = String::from("hello");
{
    let r1 = &mut s;
    println!("{}", r1);
} // r1 goes out of scope
let r2 = &mut s;
println!("{}", r2);
```

Multiple immutable references are ok:

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
println!("{}", r1);
println!("{}", r2);
```

But only as long as there is no mutable reference:

```rust
let mut s = String::from("hello");
let r1 = &s;
let r2 = &s;
let r3 = &mut s; // error, one mutable or multiple immutable references allowed
println!("{}", r1);
println!("{}", r2);
println!("{}", r3);
```

### Dangling Pointers

A function returning a pointer to a heap value that goes out of scope at the
end of that function is called a _dangling pointer_. Using dangling pointers
can crash the program and cause severe security problems. Rust doesn't allow
dangling pointers:

```rust
fn dangle() -> &String {
    let s = String::from("hello");
    &s // return a reference to an object owned by this function
} // s goes out of scope

fn main() {
    println!("{}", dangle()); // invalid: dangling ponter
}
```

The function must hand over the ownership of the objects it created for further
use:

```rust
fn dangle() -> String { // return the string (with ownership)
    let s = String::from("hello");
    s // move
}

fn main() {
    println!("{}", dangle()); // valid: owned value
}
```

## String Slices

A string slice is a reference to a part of a string. A slice stores the
starting point within the string and its length. The slice boundaries are
stated with a inclusive lower and a exclusive upper bound, so that
`upper-lower` computes to the slice length:

```rust
let s = String::from("hello world");
let hello = &s[0..5];
let world = &s[6..11];
println!("{}, {}!", hello, world); // hello, world!
```

The lower and upper bound can be omitted, defaulting to zero resp. to the
string length. If both bounds are omitted, the slice spans over the whole
string:

```rust
let s = String::from("hello world");
let hello = &s[..5]; // lower bound defaults to 0
let world = &s[6..]; // upper bound defaults to s.len()
println!("{}, {}!", hello, world); // hello, world!

let slice = &s[..]; // no bounds: span over whole string
println!("{}", slice); // hello world
```

When using multi-byte/non-ASCII characters, the slice must neither start nor
end in between the UTF-8 character boundaries:

```rust
let privet = String::from("привет");
let pri = &privet[..6]; // ok: "при"
let vet = &privet[6..]; // ok: "вет"
println!("{}{}", pri, vet);
let pri_ = &privet[..7]; // wrong: "при" + first byte of 'в'
```

A string cannot be modified when a slice is referring to it:

```rust
let mut s = String::from("abcdefg");
let abc = &s[..4];
s.push_str("hijklmnop"); // invalid: already borrowed immutably
```

Moving the immutable reference into its own scope solves the problem:

```rust
let mut s = String::from("abcdefg");
{
    let abc = &s[..4];
}
s.push_str("hijklmnop"); // valid: immutable borrow already dropped
```

String literals are string slices referring to a certain memory area within the
binary program. It's a good practice to accept string slices (type `&str`) as
function parameters, because string literals already are string slices, and
instances of `String` are cast to a string slice by referring:

```rust
fn quote(s: &str) {
    println!("«{}»", s);
}

fn main() {
    let s = String::from("Hello, World!");
    quote(&s); // String object: cast by referring
    quote("Hello, World!"); // string literal: already a slice
}
```

Slices can also be created on other sequences, such as integer arrays:

```rust
let fib = [1, 1, 2, 3, 5, 8, 13, 21];
let slice = &fib[2..7]; // [2, 3, 5, 8, 13]
```
