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
    let first_edition = Book {
        edition: 1,
        year: 2017,
        pages: 499,
        ..rust_book
    };
    let red = RGB(0xff, 0, 0);
    println!("{:?}", red);
    println!("{}", first_edition.quote());
}

impl Book {
    fn quote(&self) -> String {
        format!(
            "{}: {}. {} ({}), {}p.",
            self.author, self.title, self.publisher, self.year, self.pages
        )
    }
}
