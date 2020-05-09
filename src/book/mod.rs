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
    /// ruby_book.finish();
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
    pub fn summary(&self) {
        println!("The book `{}` is a {} pages long read and is catalogued under the ISBN {}", self.title, self.pages, self.isbn );
        self.isbn.print_qr_code();
    }

}