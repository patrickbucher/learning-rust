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
stack content is a pointer to the heap.

The content of a heap variable can be created using the `clone()` method:

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

## Move

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

A function call behaves a lot like an assignment in terms of ownership. A
function expecting a `String` will take ownership of that value, but a integer
value will just be copied:

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
