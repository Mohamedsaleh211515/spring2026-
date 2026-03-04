use std::fs::File;
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    let mut file = File::create(filename).unwrap();

    for i in 0..books.len() {
        let b = &books[i];

        let line = b.title.clone() + "," + &b.author + "," + &b.year.to_string() + "\n";

        file.write_all(line.as_bytes()).unwrap();
    }

}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function
    // Hint: Use File::open() and BufReader
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut books: Vec<Book> = Vec::new();

    for line in reader.lines() {
        let text = line.unwrap();

        let parts: Vec<&str> = text.split(",").collect();

        let title = parts[0].to_string();
        let author = parts[1].to_string();
        let year_number: u16 = parts[2].parse().unwrap();

        let new_book = Book {
            title: title,
            author: author,
            year: year_number,
        };

        books.push(new_book);
    }

    books
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}