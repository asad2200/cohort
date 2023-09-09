#![allow(dead_code)]

struct Person {
    name: String,
    age: usize
}

struct Book{
    name: String,
    author: String,
    is_available: bool
}

struct Library{
    books: Vec<Book>
}

fn main() {
    println!("Hello, world!");
}
