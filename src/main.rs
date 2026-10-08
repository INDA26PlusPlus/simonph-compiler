mod ahocorasick;
mod tokanizer;
mod parser;
use std::fs;

use tokanizer::Tokanizer;
use parser::Parser;
mod syntaxtree;

fn main() {
    let uncompiled_text = std::fs::read_to_string("examples/algebra.en").unwrap();
    let mut tokanizer = Tokanizer::new().unwrap();
    let tokens = tokanizer.tokanize(&uncompiled_text).unwrap();
    println!("printing tokens {}",tokens.len());
    for token in &tokens{
        println!("{:?}",token);
    }
    let mut parser = Parser::from_tokens(tokens);
    let program_node = parser.parse_program().unwrap();
    program_node.print_tree(0);
    print!("{}", program_node.getstring());
    fs::write("out.py", program_node.getstring()).unwrap();
}
