# Smart Pointers

- Smart Pointer: data structure that acts like a pointer with additional
  metadata and capabilities
- References borrow data, smart pointers own data.
- `String` and `Vec` _are_ smart pointers, because they own their data and have
  additional metadata (e.g. a length) and capabilities (growing as needed).
- Smart Pointers implement the `Deref` and the `Drop` trait.
    - `Deref`: allows a smart pointer instance to act like a reference
    - `Drop`: allows for custom behaviour as the pointer goes out of scope

## `Box<T>`: Storing Data on the Heap

- `Box<T>` stores its data on the heap
    - if the size of the data is unknown but required at compile time (the
      pointer has an exact size, but the pointee may vary), e.g. for recursive
      types (e.g. linked lists, trees), which require some indirection
    - tranfering ownership without copying the data
    - if only trait implementation is relevant, but not exact type
- no runtime performance overhead

## `Deref`: Treat Smart Pointers Like References

- The dereferencing operator `*` follows the reference to the value. Given the
  following bindings:
    - `let a = 4;`
    - `let b = &a;`
    - `let c = Box::new(a);`
    - Both `b` and `c` can be dereferenced as `*b` and `*c`, respectively.
    - `Box<T>` implements the `Deref` trait.
- The compiler applies _deref coercion_ where possible, so that arguments to a
  function are automatically dereferenced if needed. (E.g. a `String` argument
  can be used for a `&str` parameter
- The `DerefMut<T>` trait allows to override the `*` operator for m utable
  references.
    - Rust performs the following deref oeprations:
        - `&T` -> `&U` for `T: Deref<Target=U>`
            - from immutable to immutable
        - `&mut T` -> `&mut U` for `T: DerefMut<Target=U>`
            - from mutable to mutable
        - `&mut T` -> `&U` for `T: Deref<Target=U>`
            - from mutable to immutable
