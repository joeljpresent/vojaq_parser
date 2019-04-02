#![allow(dead_code)]

mod parser;
mod file;
mod line;
mod trio;
mod set;
mod tests;

use file::read_file;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        match read_file(args[1].as_str()) {
            Ok(set) => println!("{:?}", set),
            Err(err) => println!("{}", err)
        }
    }
}
