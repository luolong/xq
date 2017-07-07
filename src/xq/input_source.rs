use std::io::{self, Read};
use std::fs::File;
use std::path::Path;
use std::fmt::{self, Display, Formatter};

/// Source of input
///
/// Command line application accepts input from two sources:
/// - files
/// - standard input
///
/// Input sources can be _opened_ for reading.
pub enum InputSource<'a> {
    StdInput,
    Filename(&'a str)
}

impl<'a> InputSource<'a> {
    /// Create an input source from a filename
    pub fn from_path(path: &'a str) -> InputSource<'a> {
        InputSource::Filename(path)
    }

    /// Create an input source from standard input
    pub fn from_stdin() -> InputSource<'a> {
        InputSource::StdInput
    }

    /// Open input source for reading
    pub fn open(&self) -> io::Result<Box<Read>> {
        match *self {
            InputSource::StdInput => {
                let read : Box<Read> = Box::new(io::stdin());
                Ok(read)
            },
            InputSource::Filename(path) => {
                let path = Path::new(path);
                match File::open(path) {
                    Ok(file) => Ok(Box::new(file)),
                    Err(e) => Err(e)
                }
            }
        }
    }
}

impl<'a> Display for InputSource<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            InputSource::StdInput => write!(f, "std::io::stdin()"),
            InputSource::Filename(path) => write!(f, "{}", path)
        }
    }
}


