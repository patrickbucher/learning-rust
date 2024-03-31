#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    publisher: String,
    pages: u32,
    year: u32,
    edition: u8,
    price: f32,
}

#[derive(Debug)]
struct RGB(u8, u8, u8);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rust_book = Book {
        title: "The Rust Programming Language".to_string(),
        author: "Steve Klabnik and Carol Nichols".to_string(),
        publisher: "No Starch Press".to_string(),
        pages: 527,
        year: 2023,
        edition: 2,
        price: 49.99,
    };
    let mut first_edition = Book {
        edition: 1,
        year: 2017,
        pages: 499,
        ..rust_book
    };
    first_edition.pages += 3;
    println!("{}", dbg!(first_edition).quote());
    println!(
        "{}",
        Book::new(
            "Fooled by Randomness".to_string(),
            "Nassim Taleb".to_string()
        )
        .quote()
    );

    let red = RGB(0xff, 0, 0);
    println!("{:?}", red);
    println!("{}", red.code());

    let r1 = Rectangle {
        width: 4,
        height: 3,
    };
    let r2 = r1.scale(2);
    println!("{:?} {:?}", r1, r2);
    println!("{:?} {:?}", r1.area(), r2.area());
    println!("can {:?} hold {:?}? {}", r1, r2, r1.can_hold(&r2));
    println!("can {:?} hold {:?}? {}", r2, r1, r2.can_hold(&r1));
    println!("{:?}", Rectangle::new(7, 8));
    println!("{:?}", Rectangle::square(5));
}

impl Book {
    fn new(title: String, author: String) -> Self {
        Book {
            title,
            author,
            publisher: "".to_string(),
            pages: 0,
            year: 2024,
            edition: 1,
            price: 0.0,
        }
    }
}

impl Book {
    fn quote(&self) -> String {
        format!(
            "{}: {}. {} ({}), {}p.",
            self.author, self.title, self.publisher, self.year, self.pages
        )
    }
}

impl RGB {
    fn code(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
}

impl Rectangle {
    fn scale(&self, factor: u32) -> Rectangle {
        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn new(width: u32, height: u32) -> Self {
        Rectangle { width, height }
    }
    fn square(side: u32) -> Self {
        Rectangle {
            width: side,
            height: side,
        }
    }
}
