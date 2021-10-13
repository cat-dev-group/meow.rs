use std::fmt;
use std::path::Path;

/// A given Position in a file. These are usually used as start positions for a
/// Location struct.
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub index: usize,
    pub line: u32,
    pub col: u32,
}

impl Position {
    /// The first position in a file
    pub fn begin() -> Position {
        Position {
            index: 0,
            line: 1,
            col: 1,
        }
    }

    /// Increment the position 1 character forward
    pub fn advance(&mut self, passed_newline: bool) {
        if passed_newline {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        self.index += 1;
    }
}

#[derive(Debug)]
pub struct EndPosition {
    pub index: usize,
}

impl EndPosition {
    pub fn new(index: usize) -> EndPosition {
        EndPosition { index }
    }
}

/// A source location for a given Ast node or other construct.
#[derive(Debug)]
pub struct Location<'a> {
    pub filename: &'a Path,
    pub start: Position,
    pub end: EndPosition,
}

impl fmt::Display for Location<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}",
            self.filename.display(),
            self.start.line,
            self.start.col
        )
    }
}

impl<'a> Location<'a> {
    pub fn new(filename: &'a Path, start: Position, end: EndPosition) -> Location<'a> {
        Location {
            filename,
            start,
            end,
        }
    }
}

/// A trait representing anything that has a Location
pub trait Locatable<'a> {
    fn locate(&self) -> Location<'a>;
}
