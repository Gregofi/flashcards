use crate::flashcard::Flashcard;
use std::io::{BufRead, Result};

pub trait Serialize {
    fn read_file<R: BufRead>(reader: R) -> Result<Vec<Flashcard>>;
}
