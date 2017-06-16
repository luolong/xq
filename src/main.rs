use std::io::{Read, Write};
use std::fs::File;
use std::path::Path;

use std::fmt::{Display, Formatter};

#[macro_use]
extern crate clap;
use clap::{App, Values, ArgMatches};

extern crate xml;
use xml::reader::{EventReader, XmlEvent};

enum InputSource<'a> {
    StdInput,
    Filename(&'a str)
}

impl <'a> InputSource<'a> {
    fn from_path(path: &'a str) -> InputSource<'a> {
        InputSource::Filename(path)
    }
    fn from_stdin() -> InputSource<'a> {
        InputSource::StdInput
    }
}

impl <'a> Display for InputSource<'a> {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            InputSource::StdInput => write!(f, "std::io::stdout()"),
            InputSource::Filename(path) => path.display().fmt(f)
        }
    }
}


fn input_sources(values: Option<Values>)  {
    match values {
        None => {
            std::iter::once(InputSource::from_stdin()).enumerate();
        },
        Some(v) => {
            let v: Values = v;
            if v.is_empty() {
                std::iter::once(InputSource::from_stdin()).enumerate();
            }
            v.map(Path::new).map(InputSource::from_path).enumerate();
        }
    }
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
        let input: InputSource = input;
        println!("Reading from {}", input);

    }
}
