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
memory area, and thus rendering the existing references obsolete.

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

The `String` type is a collection of bytes which are interpreted as UTF-8
encoded text. Unlike the string slice type `str` (or the reference to it
`&str`), `String` is not part of the core language, but implemented in the
standard library.

A string can be created using the associated `new()` function, based on a
string literal using the associated `from()` function, or using the
`to_string()` method on an existing object implementing the `Display` trait:

```rust
let mut s1 = String::new();
let mut s2 = String::from("hello");
let mut s3 = "world".to_string();
```

Strings can be concatenated by using either the method `push_str()` (for adding
a string slice) os `push()` (for a single character) on a mutable string:

```rust
let mut s = String::new();
s.push_str("hello");
s.push(' ');
s.push_str("world");
```

Two existing strings can be combined to a third string using the `+` operator.

```rust
let foo = String::from("foo");
let bar = String::from("bar");
let qux = foo + &bar;
println!("{}", qux);
println!("{}", foo); // error: foo was consumed!
println!("{}", bar); // ok: bar was not consumed
```

Internally, a method with the signature `add(self, s: &str) -> String` is
called, which consumes the operand on the left, but doesn't own the operand on
the right. No strings are copied; ownership of the first operand is taken and
returned as the result of the concatenation.

Because chained concatenations are hard to read, the `format!` macro offers a
more convenient interface, which also doesn't take ownership of any of the
parameters:

```rust
let s1 = "hello".to_string();
let s2 = ", ".to_string();
let s3 = "world".to_string();
let message = format!("{}{}{}", s1, s2, s3);
```

Technically, the `String` type is a wraper over `Vec<u8>`. The `len()` method
returns the number of _bytes_ in the string, not _characters_:

```rust
let hello = String::from("hello");
let privet = String::from("привет");
println!("{}", hello.len());  // 5
println!("{}", privet.len()); // 12
```

The first string `"hello"` consists only of ASCII characters, which all can be
encoded with a single byte in UTF-8. The cyrillic characters of the second
string `"привет"`, however, require two bytes in UTF-8.

Strings cannot be indexed in Rust, because a index might only be refering to
some part of a encoded character, and the compiler prevents such error-prone
operations. An indexing operation on the _characters_ of a UTF-8 string would
not be possible in constant time `O(1)`, because a byte might only be _part of
a character_, and the surrounding bytes needed to considered, too.

Slicing strings, however, is legal, but causes a panic if a slice starts or
ends between two bytes belonging to the same character:

```rust
let privet = String::from("привет");
let pri = &privet[0..6];    // first three characters: при
let vet = &privet[6..12];   // last three characters: вет
println!("{}{}", pri, vet); // привет
let pri_ = &privet[0..7];   // panic: index 7 not a char boundary!
```

It's possible to iterate either over the bytes or the characters of a string:

```rust
let privet = String::from("привет");
for c in privet.chars() {
    print!("{}", c); // привет
}
println!();
for b in privet.bytes() {
    print!("{} ", b); // 208 191 209 128 208 184 208 178 208 181 209 130
}
```

## Hash Map

A `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`
using a cryptographically safe hashing function for placement and lookup.
Unlike a vector, the items cannot be retrieved by index, but by their unique
key.

The type `HashMap<K, V>` is _not_ part of the prelude and hence needs to be
made available with `use`. A new hash map can be created using the associated
`new()` method. Items can be added using the `insert(k, v)` method, which
accepts a key and a value:

```rust
use std::collections::HashMap;

let mut points = HashMap::new();
points.insert(String::from("Myers"), 125);
points.insert(String::from("Roberts"), 99);
```

Keys and values can be of different types, like `String` and `i32` in the above
example, but all keys and all values must be of the same type.

A common use case is to build up a hash map based on a list of keys and another
list of values. This can be achieved by getting an iterator on each vector
using the `iter()` method, zipping those iterators together using the `zip()`
method and calling the `collect()` method on the result:

```rust
use std::collections::HashMap;

let names = vec!["Myers".to_string(), "Roberts".to_string()];
let score = vec![125, 99];
let mut points: HashMap<_, _> = names.iter().zip(score.iter()).collect();
```

The compiler needs a type annotation for `HashMap`, but can infer the key and
value types.

Keys and values are owned by the hash map upon insertion:

```rust
let mut map = HashMap::new();
let key = String::from("Miller");
let val = 42;
map.insert(key, val);
println!("{}", key); // fail: key moved to map
```

Values of a `HashMap<K, V>` can be accessed using the `get()` method with a
key, returning an `Option<&V>` containing a reference to the value found.

```rust
let mut results = HashMap::new();
results.insert("Peter".to_string(), 7.83);
results.insert("Michael".to_string(), 6.92);
results.insert("Paul".to_string(), 8.41);

if let Some(x) = results.get("Peter") {
    println!("Peter: {}", x);
}
```

An iteration over a hash map yields key-value tuples:

```rust
let mut results = HashMap::new();
results.insert("Peter".to_string(), 7.83);
results.insert("Michael".to_string(), 6.92);
results.insert("Paul".to_string(), 8.41);

for (key, value) in &results {
    println!("{}: {}", key, value);
}
```

There can only be stored one value per key, so calling `insert(key, value)`
with a key already contained in the map overwrites that value.

```rust
let mut results = HashMap::new();
results.insert("Peter".to_string(), 7.83);
results.insert("Peter".to_string(), 6.92); // overwrite!

if let Some(x) = results.get("Peter") {
    println!("Peter: {}", x); // 6.92
}
```

Oftentimes, a new value should only be inserted into a hash map if a key is not
contained in it yet. The `entry(key)` method takes a key and returns a `Entry`:
a enum indicating an entry that might or might not exist. Its `or_insert(val)`
method inserts a value into the hash map with the key given to the initial
`entry(key)` call if an entry for that key was missing:

```rust
let mut results = HashMap::new();
results.insert("Peter".to_string(), 7.83);
results.insert("Michael".to_string(), 6.92);
results.insert("Paul".to_string(), 8.41);

results.entry("Phil".to_string()).or_insert(5.32); // new entry
results.entry("Paul".to_string()).or_insert(8.97); // existing entry
```

This interface is also useful for updating entries that already do exist. The
`or_insert(0)` call in this example is only used to make sure that there is a
starting value for the counter. The mutable reference returned by
`to_insert(0)` is then used to increment the value, which must be dereferenced:

```rust
let text = "He had had had a answer";
let mut wordcount = HashMap::new();

for word in text.split_whitespace() {
    let count = wordcount.entry(word).or_insert(0);
    *count += 1;
}
```
