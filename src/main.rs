<<<<<<< HEAD
fn main() {
    println!("Hello, world!");
=======
#[macro_use]
extern crate clap;
use clap::App;

use std::io::{self, Read};
use std::fs::File;
use std::path::Path;

extern crate xml;
use xml::reader::{EventReader, XmlEvent};

fn main() {
    
    let app = clap_app!(xq =>
        (version: crate_version!())
        (author: crate_authors!("\n"))
        (about: crate_description!())
        (@arg FILTER: +required "XML Filter pattern")
        (@arg FILES: ... "input files")
    );

    let m = app.get_matches_safe()
        .unwrap_or_else( |e| e.exit() );

    if let Some(filter) = m.value_of("FILTER") {
        println!("Filtering input elements matching {}", filter);
    }

    if let Some(files) = m.values_of("FILES") {
        for path in files.map(Path::new) {
            if path.exists() {
               println!("{}", path.display());
               if let Ok(file) = File::open(path) {
                   let parser = EventReader::new(file);
                   for e in parser {
                       println!("{:?}", e);
                   }
               }
            }
        }
    }
    else {
        println!("Reading input from stdin...");
        let parser = EventReader::new(io::stdin());
        for e in parser {
            println!("{:?}", e);
        }
    }
>>>>>>> Read XML parser events
}
