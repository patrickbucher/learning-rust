# Modules

Just like lines of code can be organized as functions, entities like functions,
enums, structs and constants can be organized in a higher logical unit:
modules.

The following examples are demonstrated using two different crates:

1. `netlib`: a library crate containing the modules
    - `src/lib.rs` as the starting point
2. `nettool`: a binary crate using the modules
    - `src/main.rs` as the starting point

The crates are created in the same directory using `cargo`:

```bash
$ cargo new netlib --lib
$ cargo new nettool --bin
```

Modules are not restricted to the usage within library crates. The purpose of
this structure is to demonstrate how a binary crate can make use of a library,
and libraries offer not only loose functions in general, but functions
organized in a module structure.

A module is declared using the `mod` keyword, following the module name and the
module definition (its content) within curly braces (`netlib/src/lib.rs)`:

```rust
mod network {
    fn connect() {}
}
```

The functions of a module are said to live in the modules _namespace_, which is
expressed by the reference `network::connect` for the above example.

Multiple modules can be declared alongside in the same file:

```rust
mod network {
    fn connect() {}
}
mod database {
    fn connect() {}
}
```

The code compiles, even though two functions called "`connect`" are declared in
the same file, because the functions are in two different modules and hence in
different namespaces.

## Hierarchy: Logical and Physical

Modules can be nested to represent hierarchic module structures:

```rust
mod server {
    fn serve() {}
    mod producer {
        fn produce() {}
    }
}
mod client {
    fn consume() {}
}
mod database {
    fn backup() {}
}
```

A file containing multiple and/or nested modules tends to become big (many
definitions) and hard to read (deeper indentation levels). It makes sense to
not only organize modules logically, but also physically in terms of files.

A module's declaration and definition can be taken apart. In case of the module
`database`, the definition remains in the file `src/lib.rs`:

```rust
mod server {
    mod producer {}
}
mod client {}
mod database;
```

The definition (consisting of a single function) is then expected to be in a
file named after the module (`src/database.rs` ):

```rust
mod database {
    fn backup() {}
}
```

During the compilation process, `lib.rs` (library crate) or `main.rs` (binary
crate), respectively, are considered for declarations. The module declaration
points the compiler forward to another file; the module `database` is looked
for in the file `database.rs` in the `src/` folder.

Extracting a nested module (`server`) with one or many child modules
(`producer`) of its own requires another organization:

1. The parent module `server` must reside in a subfolder named after the module
   `src/server/` in a file called `mod.rs`.
2. The child module `producer` must reside in the folder of its parent module
   `src/server/` in a file named after the child module `producer.rs`.

The root file `lib.rs`, with the definition of server extracted, looks like
this:

```rust
mod server;
mod client {
    fn consume() {}
}
mod database;
```

The file `server/mod.rs` only contains the server module's definitions, _not the
declaration line_ `mod server` itself, which is already implied by the context:

```rust
mod producer;
fn serve() {}
```

The submodule `server::producer` must reside in the file `server/producer.rs`:

```rust
mod producer {
    fn produce() {}
}
```

In summary, the module structure looks like this:

    netlib
        server
            producer
        database

While the file system structure looks like this:

    src/
        lib.rs
        server/
            mod.rs
            producer.rs
        database.rs

## Visibility: Private and Public

Functions and other items are private to the enclosing module by default, and
hence cannot be accessed from the outside. The `pub` keyword makes them public,
so that they can be used from the outside, too.

A public function or module is only accessible from the outside if all the
enclosing modules are declared public as well:

```rust
mod parent {
    pub mod bazzer {
        pub mod open {
            pub fn baz() {}
        }
    }
    pub mod quxer {
        mod locked { // private module
            pub fn qux() {}
        }
    }
}

fn main() {
    parent::bazzer::open::baz(); // works: whole path is public
    parent::quxer::locked::qux(); // doesn't work: private module in path
}
```

Private items—modules, functions, etc.—can only be accessed from:

1. its immediate parent module
2. the parent's child modules (siblings)
    - the parent module can be referred to using the `super` keyword in the
      module path

```rust
mod parent {
    mod first { // private module
        pub fn one() {} // public function
    }
    fn zero() {
        first::one(); // 1. immediate parent module
    }
    mod second {
        fn two() {
            super::first::one(); // 2. parent's child module (sibling)
        }
    }
}
```

Private functions that aren't used within their visible context cause a
compiler warning, because they aren't needed and therefore be better deleted.
Public functions—whether used locally or not—do not cause such a warning,
because they are intended for external use; and the external context is not
known during compilation. (A library doesn't know from where and how it is going
to be used.)

## Namespace and Scope

Big libraries with a deep module hierarchy are painful to use if every path had
to be qualified absolutely:

```rust
pub mod house {
    pub mod basement {
        pub mod freezer {
            pub mod door {
                pub fn open() {}
                pub fn close() {}
            }
        }
    }
}

fn main() {
    house::basement::freezer::door::open();
    house::basement::freezer::door::close();
}
```

The use keyword brings a module's content into scope, so that its content can
be used without further qualification.

TODO: how to apply `use` in this situation?
