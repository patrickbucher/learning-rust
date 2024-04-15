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

## `Drop`: Running Code on Cleanup

- When a value whose type implements the `Drop` trait goes out of scope,
  `drop()` is called on it.
- The `drop()` method takes a mutable reference to `self`.
- It is not allowed to call the `drop()` method manually, but indirectly by
  calling the `std::mem::drop` on the value, which will invoke its `drop()`
  method.
- The `drop()` method acts like a _destructor_ (like `finalize` in Java).

## `Rc<T>`: Reference Counted Smart Pointer

- In some data structures (e.g. graphs, double linked lists), data has multiple
  owners, and should only be cleaned up after all its owners went out of scope.
- The reference counter `Rc<T>` counts references; if zero references are
  reached, the value will be dropped.
- `Rc<T>` is only for immutable references in single-threaded scenarios.
- Use `Rc::new()` to create a new reference counter; and `Rc::clone(&x)` to
  increase the reference count to `x`. (`use std::rc::Rc`)
    - `Rc::clone` does _not_ clone all the underlying data, it only increases
      the reference count to the same data.
    - Use `Rc::strong_count(&x)` to get the reference count of `x`.
    - The strong count decreases automatically as references go out of scope.

## `RefCell<T>`: Interior Mutability Pattern

- Interior Mutability Pattern: Mutate data over immutable reference (bend Rust's
  borrowing rules). Rules are checked at runtime rather than at compile time.
- Rationale: The Rust compiler rejects some memory-safe programs; `RefCell<T>`
  allows them passing the compiler checks using `unsafe` code.
- A `RefCell<T>` can only be used in single-threaded code.
- The value inside `RefCell<T>` can be mutated even if `RefCell<T>` itself is
  immutable.
- The methods `borrow()` and `borrow_mut()` borrow the value from a `RefCell`,
  immutably and mutably, respectively. A `Ref<T>` (immutable) or `MutRef<T>` is
  returned, which both implement the `Deref` trait.
    - At runtime, multiple immutable borrows _or_ a single mutable borrow is
      allowed.

## `Rc<T>` and `RefCell<T>`: Multiple Mutable Owners

- An `Rc<T>`, which allows for multiple owners, holding a `RefCell<T>`, which
  allows for mutable borrows, allows for multiple mutable owners.

## Overview

|         | `Box<T>`            | `Rc<T>`      | `RefCell<T>`        |
|---------|---------------------|--------------|---------------------|
| Owners  | single              | multiple     | single              |
| Borrows | mutable & immutable | immutable    | mutable & immutable |
| Checks  | compile time        | compile time | runtime             |
