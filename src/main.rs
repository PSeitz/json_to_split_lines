extern crate serde;
extern crate serde_json;
use serde_json::{Deserializer, Value};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let in_file = std::env::args().nth(1).expect("require 2 command line parameter in_file and out_file");
    let out_file = std::env::args().nth(2).expect("require 2 command line parameter in_file and out_file");
    println!("{:?}", split_json_to_lines(&in_file, &out_file));
}

fn split_json_to_lines(input:&str, output:&str) -> Result<(),std::io::Error> {

    let f = File::open(input)?;
    let json_stream = Deserializer::from_reader(std::io::BufReader::new(&f)).into_iter::<Value>();
    let mut out_file = File::create(output)?;

    for el in json_stream {
        if let Some(arr) = el.as_ref().unwrap().as_array() {
            for el in arr.iter() {
                out_file.write_all(el.to_string().as_bytes())?;
                out_file.write_all("\n".as_bytes())?;
            }
        }else{
            out_file.write_all(input.to_string().as_bytes())?;
            out_file.write_all("\n".as_bytes())?;
        }
    }
    Ok(())
}

