# Common Collections

Rust has different kinds of collections, which allow to store multiple elements
of the same type. The elements are stored on the heap, and therefore, unlike
arrays, the number of items is flexible.

## Vector

The type `Vec<T>` describes a _vector_, which stores a list of items in a
continuous memory area. A vector can be created using its associated `new()`
function:

```rust
let v: Vec<i32> = Vec::new();
```

A type annotation (within angle brackets) is needed, because there are no
elements yet to infer the type from. If a vector is to be created from existing
items, the `vec!` macro (included in the prelude) is more convenient:

```rust
let v = vec![1, 2, 3];
```

Elements can be added to a mutable vector using the `push()` method:

```rust
let mut v = vec![1, 2, 3];
v.push(4);
v.push(5);
v.push(6);
```

There are two ways of reading the elements of a vector `v`:

1. Using _indexing syntax_ `&v[i]`, which panics if an invalid index is used.
2. Using the `v.get(i)` method, which returns an `Option<T>` with either a
   value `Some(T)` for valid indices or the `None` variant for invalid indices.

```rust
let v = vec![1, 2, 3];
let first = &v[0];     // 1: i32
let second = v.get(1); // Some(2): Option<i32>
let fourth = &v[3];    // panic!
let fifth = v.get(4);  // None: Option<i32>
```

Storing a reference of a vector item is borrowing from the vector. As long as
a borrow is held (and used), the vector cannot be modified by adding items to
it:

```rust
let mut v = vec![1, 2, 3];
let one = &v[0];     // immutable borrow
v.push(4);           // mutable borrow
println!("{}", one); // use immutable borrow later
```

The reason for this restriction is the way the memory of a vector is organized:
If the vector no longer fits into its current memory area after an additional
item is added, the _whole_ vector needs to be moved to a bigger continuous
memory are, and thus rendering the existing references obsolete.

The items of a vector can be iterated over using the `for`/`in` loop:

```rust
let v = vec![1, 2, 3];
for i in v {
    println!("{}", i);
}
```

The items of a mutable vector can be modified in an iteration, if the vector is
borrowed mutably for the operation, and the item is dereferenced upon
modification:

```rust
let mut v = vec![1, 2, 3];
for i in &mut v {
    *i *= 2;
}
```

The `pop()` method returns the last item of a vector wrapped in an `Option<T>`
and removes the item from the vector:

```rust
let mut v = vec![1, 2, 3, 42];
if let Some(i) = v.pop() {
    println!("last: {}", i);
}
println!("rest: {} {} {}", v[0], v[1], v[2]);
```

Vectors can store different variants of an enum, for they belong to the same
type:

```rust
enum DivisionResult {
    Integer(i32),
    FloatingPoint(f64),
    Undefined,
}

let results = vec![
    DivisionResult::Integer(3),         // 6/2
    DivisionResult::FloatingPoint(3.5), // 7/2
    DivisionResult::Undefined,          // 3/0
];

for r in results {
    match r {
        DivisionResult::Integer(i) => println!("{}", i),
        DivisionResult::FloatingPoint(f) => println!("{}", f),
        DivisionResult::Undefined => (),
    }
}
```

## String

TODO

## Hash Map

TODO
