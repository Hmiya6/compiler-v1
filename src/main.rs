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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_node() {
        test_tokenize("1*(2+3)");
        test_tokenize("1 + 20+ 4");
        test_tokenize(" 9- 6 * 10");
        test_tokenize("1-10/100 +1000 * 10000");
        test_tokenize("((2-20)*200 + 2000)*(21 - 201)");
        test_tokenize("((100 + 100)* 10) + 100");
        test_tokenize("1 == 1");
        test_tokenize("1 != 1");
        test_tokenize("1 <= 1");
        test_tokenize("1 >= 1");
        test_tokenize("1 < 1");
        test_tokenize("1 > 1");
        test_tokenize("1 == 1 == 1");
        test_tokenize("1 > 1 > 1");
    }
    
    #[test]
    fn test_compile() {
        CodeGenerator::from_str("((100 + 100)* 10) + 100").compile();
        CodeGenerator::from_str("-5").compile();
        CodeGenerator::from_str("123 +  (  + 33 - 99 )* 24").compile();
        CodeGenerator::from_str("123 > 122").compile();
        CodeGenerator::from_str("42 == 43").compile();
    }

    fn print_node(node: &Node) {
        println!("{:?}", node.kind);
        if let Some(n) = &node.lhs {
            print_node(n);
        }
        
        if let Some(n) = &node.rhs {
            print_node(n);
        }
    }

    fn test_tokenize(s: &str) {
        let mut input = Input::new(s);
        let head = input.tokenize();
        print_node(&head);
        println!("------------");
    }
}


