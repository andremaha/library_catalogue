extern crate library_catalogue;

use library_catalogue::book::{Book};
use library_catalogue::book::isbn::{ISBN};
use library_catalogue::book::read::{Read};
use library_catalogue::shelf::{Shelf};

fn main() {

    let rust_book = Book {
        title: String::from("The Rust Programming Language"),
        isbn: ISBN::V10(978, 1, 59327, 828, 1),
        pages: 519,
        read: Read::Started
    };


    let javascript_book = Book {
        title: String::from("Eloquent JavaScript"),
        isbn: ISBN::V13(978, 1, 59327, 564, 6),
        pages: 451,
        ..rust_book
    };

    let mut ruby_book = book_factory(String::from("Beginning Ruby: From Novice to Professional"), 
                                 ISBN::V13(978, 1, 43022, 363, 4),
                                 620);

    ruby_book.finish();


    let mut programming_shelf = Shelf::named(String::from("Programming Languages"));

    programming_shelf.put_book(rust_book);
    programming_shelf.put_book(javascript_book);
    programming_shelf.put_book(ruby_book);

    list_shelf_contents(Some(&programming_shelf));

    let empty_shelf = None;

    list_shelf_contents(empty_shelf);
    

}


/// Creates an instance of a book
/// 
/// # Example
/// 
/// ```
/// let mut ruby_book = book_factory(String::from("Beginning Ruby: From Novice to Professional"), 
///                                 ISBN::V13(978, 1, 43022, 363, 4),
///                                 620);
/// ```                      
fn book_factory(title: String, isbn: ISBN, pages: u32) -> Book {
    Book {
        title,
        isbn,
        pages,
        read: Read::NotStarted
    }
}

/// Shows what's on the shelf
/// 
/// Names the shelf and then all the books on it
fn list_shelf_contents(shelf: Option<&Shelf>) {

    match shelf {
        None => {
            println!("Seems that the shelf is not there (yet?)");
        },
        Some(shelf) => {
            println!("On the `{}` shelf there are following volumes:", shelf.name);

            for book in shelf.books.iter() {
                book.summary();
                match book.read {
                    Read::Finished => println!("+ And I have finsihed that one!"),
                    _ => {}
                }
            }
        }
    }
}