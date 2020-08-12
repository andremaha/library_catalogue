//! # Library Catalogue
//! 
//! `library_catalogue` is a collection of utilities to track the books in your library. 
//! Yes, physical books (with covers, pages and lots of space to do marking with your highligher) 
//! in your library (shelves that stand up right in your study).

// Re-exporting the modules to create a convenient public API
pub use crate::book::Book;
pub use crate::book::isbn::ISBN;
pub use crate::book::read::Read;
pub use crate::shelf::Shelf;

pub mod book;
pub mod shelf;


