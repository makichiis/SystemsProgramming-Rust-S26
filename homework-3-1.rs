#![feature(string_replace_in_place)]

use std::fs::File;
use std::fmt::Write;
use std::io::{Write as OtherWrite, BufReader, BufRead};

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    year: u16,
}

trait CSVSerializable {
    fn from_csv(csv_string: &str) -> Result<Self, String> where Self: Sized;
    fn into_csv(&self) -> String;
}

fn get_csv_escape(s: &String) -> String {
    s.replace(",", "\\,")
}


fn get_csv_fields(csv_string: &str) -> Vec<String> {
    let mut fields: Vec<String> = vec![];
        
    let mut escapes: bool = false;
    for tok in csv_string.split(",") {
        if escapes && !fields.is_empty() {
            let _ = write!(fields.last_mut().unwrap(), "{}", tok.to_string());
            escapes = false;
        } else {
            fields.push(tok.to_string());
        }

        if fields.last().unwrap().chars().last() == Some('\\') {
            escapes = true;
            fields.last_mut().unwrap().replace_last("\\", ",");
        }
    }

    fields 
}

impl CSVSerializable for Book {
    fn from_csv(csv_string: &str) -> Result<Self, String> {
        let fields: Vec<String> = get_csv_fields(csv_string);

        if fields.len() != 3 {
            return Err(format!("Invalid field count. Expected 3, got: {} from '{}'", fields.len(), csv_string));
        }

        let title = fields[0].clone();
        let author = fields[1].clone();
        let year = fields[2].parse::<u16>();
        if year.is_err() {
            return Err(format!("Invalid value for field `year`: expected an integer, got {}", fields[2].clone()));
        }
        let year = year.unwrap();

        Ok(Book{ title: title, author: author, year: year })
    }

    fn into_csv(&self) -> String {
        let title_escape = get_csv_escape(&self.title);
        let author_escape = get_csv_escape(&self.author);

        format!("{},{},{}", title_escape, author_escape, self.year)
    }
}

fn save_books(books: &Vec<Book>, filename: &str) {
    let entries: Vec<String> = books.into_iter().map(|book: &Book| book.into_csv()).collect();

    let mut file = File::create(filename).unwrap();

    for entry in entries {
        let _ = writeln!(file, "{}", entry);
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    let mut retval: Vec<Book> = vec![];

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    // TODO: detect content newlines 
    for entry in reader.lines() {
        retval.push(Book::from_csv(&entry.unwrap()).unwrap());
    }

    retval 
}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
        Book { title: "Me, Myself, and I".to_string(), author: "G-Eazy".to_string(), year: 2015 }, 
        //Book { title: "icky\ntitle".to_string(), author: "null".to_string(), year: 2055 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}
