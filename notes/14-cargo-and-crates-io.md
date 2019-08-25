# Cargo and Crates.io

The Cargo toolchain and the crate repository [Crates.io](https://crates.io)
form a strong eco-system around the programming language. Cargo's
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
accurate meta-data, a useful API, and a comprehensive test-suite.

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

### Reexports

### Crate Metadata

### Publishing the Crate

## Workspaces

## Installing Binaries

## Extending Cargo
