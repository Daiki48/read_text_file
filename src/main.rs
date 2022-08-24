use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let file = File::open("./src/data/test.txt").unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines(){
        println!("{}", line.unwrap());
    }
}
