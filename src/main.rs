use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;
use std::iter::Iterator;

use std::fmt::{Display, Formatter};

#[macro_use]
extern crate clap;
use clap::{Values, ArgMatches};

extern crate xml;
use xml::reader::EventReader;

enum InputSource<'a> {
    StdInput,
    Filename(&'a str)
}

impl<'a> InputSource<'a> {
    fn from_path(path: &'a str) -> InputSource<'a> {
        InputSource::Filename(path)
    }
    fn from_stdin() -> InputSource<'a> {
        InputSource::StdInput
    }
    fn open(&self) -> std::io::Result<Box<Read>> {
        match *self {
            InputSource::StdInput => {
                let read : Box<Read> = Box::new(std::io::stdin());
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
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            InputSource::StdInput => write!(f, "std::io::stdin()"),
            InputSource::Filename(path) => write!(f, "{}", path)
        }
    }
}

fn input_sources(values: Option<Values>) -> Vec<InputSource> {
    if let Some(values) = values {
        let values : Vec<InputSource> = values.map(InputSource::from_path).collect();
        if !values.is_empty() {
            return values;
        }
    }
    return vec!(InputSource::from_stdin());
}



fn main() {
    
    let app = clap_app!(xq =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg FILTER: +required "XML Filter pattern")
        (@arg FILES: ... "input files")
    );

    let m: ArgMatches = app.get_matches_safe()
        .unwrap_or_else(|e| e.exit());

    if let Some(filter) = m.value_of("FILTER") {
        println!("Filtering input elements matching {}", filter);
    }

    let inputs = input_sources(m.values_of("FILES"));
    for input in inputs {
        let input = Box::new(input);
        println!("Reading from {}", input);

        match input.open() {
            Ok(source) => {
                let events = EventReader::new(source);
                for event in events {
                    writeln!(std::io::stdout(), "XML Event: {:?}", event).unwrap();
                }
            }
            Err(e) => {
                writeln!(std::io::stderr(), "Could not open {}: \n {}", input, e).unwrap();
            }
        }
    }


}
