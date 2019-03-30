#![allow(dead_code)]

mod parser;
mod vojaq_file;
mod vojaq_line;
mod vojaq_trio;
mod vojaq_set;
mod tests;

use vojaq_file::read_file;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        match read_file(args[1].as_str()) {
            Ok(set) => {
                println!("{:?}", set);
                println!("Prononciation de 1337 : {:?}", 
                    set.trios_ref().get(3).unwrap().secondo_ref())
            },
            Err(err) => println!("{}", err)
        }
    }
}
