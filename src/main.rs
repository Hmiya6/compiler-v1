pub mod utils;
pub mod lexer;
pub mod node;
pub mod codegenerator;

use crate::codegenerator::CodeGenerator;
use std::env;
// use anyhow::{anyhow, Result};



fn main() {
    // read command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Invalid number of command line arguments");
    }

    // compile
    let mut compiler = CodeGenerator::from_str(&args[1]);
    compiler.compile();
}



