use crate::book::Book;

#[derive(Debug)]
pub struct Shelf {
    pub name: String,
    pub books: Vec<Book>
}

impl Shelf {

    /// Puts a book on the shelf
    ///
    /// # Example
    /// 
    /// ```
    ///  use library_catalogue::shelf::Shelf;
    ///  use library_catalogue::book::Book;
    ///  use library_catalogue::book::isbn::ISBN;
    ///  use library_catalogue::book::read::Read;
    ///  
    ///  let rust_book = Book {
    ///    title: String::from("The Rust Programming Language"),
    ///    isbn: ISBN::V10(978, 1, 59327, 828, 1),
    ///    pages: 519,
    ///    read: Read::Started
    ///  };
    ///
    ///  let mut programming_shelf = Shelf::named(String::from("Programming Languages"));
    ///  programming_shelf.put_book(rust_book);
    /// ```
    pub fn put_book(&mut self, book: Book) {
        self.books.push(book);
    }

    /// Initiates a new shelf
    /// 
    /// # Example
    /// 
    /// ```
    /// use library_catalogue::shelf::Shelf;
    /// let programming_shelf = Shelf::named(String::from("Programming Languages"));
    /// ```
    pub fn named(name: String) -> Shelf {
        Shelf {
            name,
            books: Vec::new()
        }
    }
}