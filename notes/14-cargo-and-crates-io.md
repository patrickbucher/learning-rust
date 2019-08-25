# Cargo and Crates.io

The Cargo toolchain and the crate repository [Crates.io](https://crates.io)
form a strong ecosystem around the programming language. Cargo's
[documentation](https://doc.rust-lang.org/cargo/) covers all the details.

## Customizing Builds

A _release profile_ is a set of configuration for code compilation. By default,
there's a `dev` and a `release` profile. The `dev` profile is used when `cargo
build` is invoked without any special flag. The `release` profile can be used
when setting the `--release` flag. The profile used will be shown in the
output:

	$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s

	$ cargo build --release
    Finished release [optimized] target(s) in 0.00s

The details for every profile can be defined in `Cargo.toml`, by defining a
section for the profile of interest:

```toml
[profile.dev]
opt-level = 1

[profile.release]
opt-level = 2
```

This configuration sets the optimization level of the `dev` profile to 1 (as
opposed to the default of 0) and for the `release` profile to 2 (default: 3),
just for the sake of demonstration.

Having the lowest optimization level for `dev` and the highest for `release` is
a sensible configuration, for during development the build should be fast, but
the execution time of the binary hardly matters, whereas in production the
compilation time is hardly an issue, while execution speed of the resulting
binary is critical.

## Publishing Crates

[Crates.io](https://crates.io) is not only a one-way channel to retrieve code
from. It is also a place where developers can share their own code. In order to
be useful to other developers, the code must provide useful documentation,
accurate metadata, a useful API, and a comprehensive test-suite.

Those requirements are best demonstrated using an example: The library crate
`mememo`, which is short for _mean, median, and mode_`, implements trivial
implementations for the statistical concepts it is named after.

### The `mememo` Crate

The `mememo` crate provides three public functions, which all expect a vector
of integers (`i32`):

1. `mean`: Calculates the mean, i.e. sums up the elements of the vectors and
   divides the sum by the number of elements.
2. `median`: Returns the value of the middle element of the given vector.
3. `mode`: Returns the item that has the most occurrences in the given vector.

`mean` returns a `f64`, because a division of a sum is not to be expected being
a discrete number. `mode` returns an integer, because the result of this
operation is taken from the input vector. For `median`, there are two possible
outcomes:

- If the number of elements in the input vector is _odd_, the middle element of
  the vector is the result, and thus the return type is the same as the input
  vector's element type: `i32`.
- If the number of elements in the input vector is _even_, the median is
  calculated as the mean of the two middle elements. In this case, the result
  is a `f64`.

These possible outcomes can be expressed using an enum with two variants:

```rust
pub enum Median {
    MiddleSingle(i32),
    MiddleTwoMean(f64),
}
```

The function headers of the three functions look as follows:

```rust
pub fn mean(numbers: &Vec<i32>) -> f64 {
	// ...
}

pub fn median(numbers: &Vec<i32>) -> Median {
	// ...
}

pub fn mode(numbers: &Vec<i32>) -> i32 {
	// ...
}
```

The implementations are omitted and can be seen in the file
`code/mememo/src/lib.rs`.

### Documentation Comments

Whereas ordinary comments clarify details on specific sections of code,
_documentation comments_ are intended for the user of the public API rather
than for a maintainer of the code.

Rust code is commented using two slashes (`//`), and those comments can occur
anywhere in the code. Documentation comments start with three slashes (`///`)
and must be located just before the element they're documenting. Those comments
should indicate how to use the public item at hand. For this purpose, Markdown
syntax can be used, as well as code examples.

Here's the documentation comment of the `mean` function discussed above:

```rust
/// Calculates the mean of the elements in the given vector.
///
/// # Example
///
/// ```
/// let numbers = vec![1, 2, 3, 4];
/// assert_eq!(2.5, mememo::mean(&numbers));
/// ```
pub fn mean(numbers: &Vec<i32>) -> f64 {
	// ...
}
```

The `cargo doc` tool generates HTML documentation based on these comments and
puts it into the `target/doc` folder. The documentation can be generated an
opened in a browser by invoking `cargo doc --open`.

It is important that the code examples used in documentation comments do
actually compile ‒ and pass the assertions used. Broken example code is
annoying, and `cargo test` makes sure, the example code is working code, by
executing the code as `Doc-tests`:

	$ cargo test

	...

	Doc-tests mememo
	test src/lib.rs - mean (line 11) ... ok
	test src/lib.rs - mode (line 65) ... ok
	test src/lib.rs - median (line 33) ... ok
	test src/lib.rs - median (line 44) ... ok

	test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

Having an _Example_ section is common for Rust crates. Other sections commonly
provided are:

- Panics: scenarios in which the code will panic (to be avoided by the caller)
- Errors: kinds of errors being returned and their underlying conditions
- Safety: invariants the caller needs to hold up when invoking `unsafe` code

In order to document the item that contains the comment, the comment style
`//!` can be used. For example, the entire `mememo` crate (`src/lib.rs`) can be
documented as follows:

```rust
//! # mememo
//!
//! The crate _mememo_ provides trivial implementations of the operations
//! _mean_, _median_, and _mode_. This crate is not intended for productive
//! use, but only to demonstrate the use of crates and other Rust features.

use std::collections::HashMap;

/// Calculates the mean of the ...
```

There's no code following this comment (the import section only starts after an
empty line). There's another documentation comment below, which belongs to the
`mean` function further down the file.

These kinds of comments appear on the front page of the crate's documentation.

### Re-exports

The internal structure of the code often does not provide the most convenient
API possible. The code examples of the `median` function serve as good example
for the verbosity an API user is facing:

```rust
let numbers = vec![1, 2, 3, 4, 5];
match mememo::median(&numbers) {
    mememo::Median::MiddleSingle(got) => assert_eq!(3, got), // verbose!
    _ => panic!("wrong median calculation"),
}
let numbers = vec![1, 2, 3, 4];
match mememo::median(&numbers) {
    mememo::Median::MiddleTwoMean(got) => assert_eq!(2.5, got), // verbose!
    _ => panic!("wrong median calculation"),
};
```

It would be much more convenient if the client could write
`mememo::MiddleSingle` or `mememo::MiddleTwoMean` instead of always having to
add the `Median` in between.

Rust allows to re-export such items under a more convenient name using the `pub
use` notation. The internal structure of the code is preserved, but the client
has it much easier to deal with the exposed API.

The enum's variants can be re-exported as follows:

```rust
pub use Median::MiddleSingle;
pub use Median::MiddleTwoMean;
```

These re-exports are now displayed in an additional section (_Re-exports_) in
the documentation. The API can now be used as follows:

```rust
let numbers = vec![1, 2, 3, 4, 5];
match mememo::median(&numbers) {
    mememo::MiddleSingle(got) => assert_eq!(3, got), // less verbose!
    _ => panic!("wrong median calculation"),
}
let numbers = vec![1, 2, 3, 4];
match mememo::median(&numbers) {
    mememo::MiddleTwoMean(got) => assert_eq!(2.5, got), // less verbose!
    _ => panic!("wrong median calculation"),
};
```

### Crate Metadata

When a new crate is created using `cargo new`, some metadata is automatically
put into `Cargo.toml`, such as the crate's name, the initial version, and the
author's details (which are obtained from the `git` config). In order to
publish a crate on [Crates.io](https://crates.io), at least two additional
fields need to be defined:

1. `description`: a short description (one or two sentences) about the crate.
   This description will be displayed in the search results on Crates.io.
2. `license`: a _license identifier value_ specifying the license the code is
   released under. Those identifiers can be obtained from the Linux
   Foundation's Software Package Data Exchange (SPDX) under
   [spdx.org/licenses](https://spdx.org/licenses/). Multiple licenses can be
   listed separated by `OR`.

For example, `Cargo.toml` of `mememo` with an added `description` and dual
licensing (MIT and GPLv3 or later) looks as follows:

```toml
[package]
name = "mememo"
version = "0.1.0"
description = "mememo stands for Mean, Median, Mode. It provides trivial implementations for those operations."
authors = ["Patrick Bucher <patrick.bucher@stud.hslu.ch>"]
edition = "2018"
license = "MIT OR GPL-3.0-or-later"

[dependencies]
```

### Publishing the Crate

In order to publish a crate, an account on [Crates.io](https://crates.io) is
needed, which currently requires a [GitHub](https://github.com) account. After
the account is created, a new API key can be generated and obtained under
[crates.io/me](https://crates.io/me). This key can be used to login to
[Crates.io](https://crates.io) from the local computer:

	$ cargo login abcdefghijklmnopqrstuvwxyz012345

This key is stored locally under `~/.cargo/credentials` and is considered a
_secret_, thus must not be shared with others. To logout, simply delete the key
from the `credentials` file.

Before publishing code, it is import to consider that a publish is permanent.
Existing code can be neither deleted nor owerwritten.  It is only possible to
publish additional code with a higher version number.

When the crate is ready for publishing ‒ with a sufficient code-quality,
without known critical bugs, with a useful and well-documented public API, a
test-suite, and the necessary metadata ‒ the crate can be published:

	$ cargo publish

To publish a new (higher) version of an existing crate, simply increase the
`version` metadata field in `Cargo.toml` according to the rules of Semantic
Versioning ([semver.org](https://semver.org/)) and then call `cargo publish`
again. It is a good idea to also tag the version in the SCM system, e.g. using
`git tag v0.1.1` to add the version tag `v0.1.1` to current committed state of
the repository.

### Un-publishing Versions

Even though it is not possible to remove crates or specific versions thereof
from [Crates.io](https://crates.io), it is possible to prevent future use of
specific versions of the crate using the `cargo yan` command.

Let's say the version `0.1.0` is buggy, and therefore a later version `0.1.1`
was released fixing those bugs, future use of the version `0.1.0` can be
prevented as follows:

	$ cargo yank --vers 0.1.0

If an existing project already uses that version and thus has an entry for it
in `Cargo.lock`, the project still builds, and the version `0.1.0` can still be
used from that project. New projects, however, are required to use the later
version `0.1.1` as a dependency ‒ unless the the yank operation is undone,
allowing future use for the specified version again:

	$ cargo yank --vers 0.1.0 --undo

## Workspaces

TODO

## Installing Binaries

TODO

## Extending Cargo

TODO
