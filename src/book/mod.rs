use crate::book::isbn::ISBN;
use crate::book::read::Read;

pub mod isbn;
pub mod read;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub isbn: ISBN,
    pub pages: u32,
    pub read: Read
}

impl Book {

    /// Marks a book as finished
    /// 
    /// # Affected fields
    /// 
    /// - read
    /// 
    /// # Example
    /// 
    /// ```
    ///  use library_catalogue::book::Book;
    ///  use library_catalogue::book::isbn::ISBN;
    ///  use library_catalogue::book::read::Read;
    ///  
    ///  let mut rust_book = Book {
    ///    title: String::from("The Rust Programming Language"),
    ///    isbn: ISBN::V10(978, 1, 59327, 828, 1),
    ///    pages: 519,
    ///    read: Read::Started
    ///  };
    ///  
    /// rust_book.finish();
    /// ```
    pub fn finish(&mut self) {
        self.read = Read::Finished;
    }

    /// Prints a summary of the book
    /// 
    /// # Included fields
    /// - title
    /// - pages
    /// - isbn
    pub fn summary(&self) -> String {
        format!("The book `{}` is a {} pages long read and is catalogued under the ISBN {}\n{}", self.title, self.pages, self.isbn, self.isbn.print_qr_code())
    }

}