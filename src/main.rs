use std::io::prelude::*;

#[macro_use]
extern crate clap;
use clap::{Values, ArgMatches};

extern crate xml;
use xml::reader::EventReader;

mod xq;
use xq::input_source::InputSource;


fn input_sources<'a>(values: Option<Values>) -> Vec<InputSource> {
    let paths = values.and_then( |values| {
        let values : Vec<InputSource> = values.map(InputSource::from_path).collect();
        if values.is_empty() { None } else { Some(values) }
    });
    paths.unwrap_or(vec!(InputSource::from_stdin()))
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
