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
    /// let programming_shelf = Shelf::named(String::from("Programming Languages"));
    /// ```
    pub fn named(name: String) -> Shelf {
        Shelf {
            name,
            books: Vec::new()
        }
    }
}