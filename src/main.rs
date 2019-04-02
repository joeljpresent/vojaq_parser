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
            Ok(set) => {
                println!("{:?}", set);
                println!("Prononciation de 1337 : {:?}", 
                    set.trios_ref().get(3).unwrap().secondo_ref())
            },
            Err(err) => println!("{}", err)
        }
    }
}
