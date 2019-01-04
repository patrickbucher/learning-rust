# Structs

Structs name and group together multiple related values of possibly different
types to a new type. Unlike tuples, structs give names to the whole and its
parts, and the order of fields neither matters for initialization nor accessing
values.

A struct is defined using the keyword `struct`, a name (in capitals) and a list
of fields (name-type pairs) in curly braces:

```rust
struct Employee {
    name: String,
    position: String,
    logins: u64,
    active: bool,
}
```
A new instance of a struct is created by defining its field in key-value
notation. All fields are mandatory and need to be initialized:

```rust
let dilbert = Employee {
    name: String::from("Dilbert"),
    position: String::from("Engineer"),
    logins: 962,
    active: true,
};
```

When marked as mutable, fields of a struct instance can be changed:

```rust
let mut dilbert = Employee {
    name: String::from("Dilbert"),
    position: String::from("Engineer"),
    logins: 962,
    active: true,
};
dilbert.name = String::from("Dilberto");
dilbert.position = String::from("Head of Engineering");
dilbert.logins += 1;
dilbert.active = false;
```

Only the struct instance as a whole can be marked as mutable, not single
fields.

A struct without any fields is called a _unit-like_ struct. It can be useful
where a value is needed formally, but doesn't matter:

```rust
struct Empty{};
let container = Empty{};
```

Factory functions are helpful to create new struct instances, especially if
they consist of a mix of custom and default values:

```rust
fn create_employee(name: String, position: String) -> Employee {
    Employee {
        name: name,
        position: position,
        logins: 0,
        active: true,
    }
}
```

If the field and the variable assigned to it have the same name, the _field
init shorthand_ syntax allows to only indicate the name once:

```rust
fn create_employee(name: String, position: String) -> Employee {
    Employee {
        name,
        position,
        logins: 0,
        active: true,
    }
}
```

This not only works for function parameters, but for any variables in scope:

```rust
let name = String::from("Catbert");
let position = String::from("Evil Genius");
let catbert = Employee {
    name,
    position,
    logins: 0,
    active: true,
};
```

New instances can be created based on existing ones:

```rust
let wally = Employee {
    name: String::from("Wally"),
    position: dilbert.position,
    logins: 312,
    active: dilbert.active,
};
```

The _struct update_ syntax allows to initialize the remaining fields based on
the value of an existing instance:

```rust
let wally = Employee {
    name: String::from("Wally"),
    logins: 312,
    ..dilbert // use Dilbert's values for fields not yet initialized
};
```

## Tuple Structs

A tuple struct is a tuple with a name. The name belongs to the type definition,
therefore this two tuple structs are not compatible to each other:

```rust
struct RGB(u8, u8, u8);
struct Block(u8, u8, u8);

let red = RGB(255, 0, 0);
let cube = Block(1, 1, 1);
```

A loose bunch of variables on one side and a struct on the other side can be seen as the two extremes of a continuum:

1. _loose variables_ can be grouped together to a _tuple_
2. a _tuple_ can be named as a whole to get a _named tuple_
3. the fields of a _named tuple_ can be named to get a _struct_

## Debug Output

Struct instances can be printed out if they derive the trait `std::fmt::Debug`:

```rust
#[derive(Debug)]
struct Employee {
    name: String,
    position: String,
    logins: u64,
    active: bool,
}

fn main() {
    let dilbert = Employee {
        name: String::from("Dilbert"),
        position: String::from("Engineer"),
        logins: 962,
        active: true,
    };
    println!("{:?}", dilbert);
    println!("{:#?}", dilbert);
}
```

The `{:?}` output format prints the struct on a single line, whereas the
`{:#?}` output format uses multiple lines and indents the fields:

    Employee { name: "Dilbert", position: "Engineer", logins: 962, active: true }
    Employee {
        name: "Dilbert",
        position: "Engineer",
        logins: 962,
        active: true
    }

## Methods

A method is a function defined in the context of a struct. The instance of the
struct the method is called on is automatically provided as the first parameter
called `self`. The methods of a struct must be declared within one or multiple
`impl` blocks:

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
```
The `area` method accepts a reference to a `Rectangle` (the type doesn't need
to be declared, because it can be inferred). The method can be called using dot
notation on the struct instance (the reference operator `&` is optional):

```rust
fn main() {
    let r = Rectangle {
        width: 3,
        height: 4,
    };
    println!("{}", r.area()); // 12
    println!("{}", &r.area()); // with optional reference operator
}
```

The `self` instance can be modified by accepting a mutable reference:

```rust
impl Rectangle {
    fn stretch(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    let mut r = Rectangle {
        width: 3,
        height: 4,
    };
    println!("{}", r.area()); // 3 * 4 = 12
    r.stretch(2);
    println!("{}", r.area()); // 6 * 8 = 48
}
```

Methods that own the `self` parameter are rare. They're helpful when a new
instance is created based on an old one, and the old instance must no longer be
used afterwards. The old instance is _consumed_ by the method:

```rust
impl Rectangle {
    fn transform(self, factor: u32) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

fn main() {
    let r = Rectangle {
        width: 3,
        height: 4,
    };
    let r = r.transform(2);
    println!("{}", r.area());
}
```

Associated functions belong to a struct, but do not take a `self` parameter;
they are often used as constructors:

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

An associated function can be called using double colon notation on the struct,
as already used for `String::from()` before:

```rust
fn main() {
    let r = Rectangle::square(3);
    println!("w: {}, h: {}", r.width, r.height); // w: 3, h: 3
}
```
