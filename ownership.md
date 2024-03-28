# Notes on Ownership

## From the Book (Original)

- each value has one owner (variable)
- `drop()` is automatically called on any variable that goes out of scope
    - since each value has a single owner, no _double free_ errors will occur
- for value types (primitives), assignment _copies_ the value
    - if the type implements the `Copy` trait
    - the arguments of a function call and returning a value behave like an assignment
- for reference types (on the heap), assignment _moves_ ownership
- a _reference_ is a pointer without ownership
    - it always must be valid
    - a reference "borrows" a value
    - a referenced value won't be dropped when the reference variable goes out
      of scope
    - rule: one mutable or multiple immutable references per value are allowed

## From the Brown University's Version

- undefined behaviour (e.g. use of a undeclared variable)
    - safety: no undefined behaviour
    - see list of undefinded behaviour on [Rust Docs](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
- stack frame: function scope (and block scope?)
- pointer/pintee: stack -> heap
- heap data: `Box::new(VALUE)`
    - deallocated as pointer owning the heap data goes out of scope
- pointing to deallocated memory is _not_ a problem
    - but dereferencing that pointer _is_ a problem
- `*` is the dereference operator (e.g. for a `Box`)
- aliasing: access the same data through different variables or "paths"
    - aliasing + mutation = danger
    - data freed over one pointer, dereferenced through another pointer
    - data updated (concurrently): inconsistencies, race conditions
- pointer safety principle: alias _or_ mutation
- variables are "access paths" to data with permissions
    - R (read), W (write), and O (own)
    - capabilities: R: copy, W: mutate, O: move/drop
    - those permissions only exist during compile-time
    - `let` -> RO
    - `let mut` -> RWO 
    - references temporarely remove permissions from the owning variables
    - permissions belong to _paths_, not to variables
- data must outlive all references to it!

## Question

- "Referencing a variable is a borrow?"
