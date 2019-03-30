#![allow(dead_code)]

mod parser;
mod vojaq_file;
mod vojaq_line;
mod vojaq_trio;
mod vojaq_set;
mod tests;

use parser::*;
use vojaq_line::*;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        match parse_line(args[1].as_str()) {
            VojaqLine::Trio(trio) => println!("Le trio vaut {:?}.", trio),
            voj => println!("Voj : {:?}", voj)
        }
    }
}
