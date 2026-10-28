mod ahocorasick;
mod tokanizer;
mod parser;
use std::fs;
use std::env;
use tokanizer::Tokanizer;
use parser::Parser;
mod syntaxtree;

fn main() {
    let args:Vec<String> = env::args().collect();
    let filedest:&str = &args[1];
    let uncompiled_text = std::fs::read_to_string(filedest).unwrap();
    let mut tokanizer = Tokanizer::new().unwrap();
    let tokens = tokanizer.tokanize(&uncompiled_text).unwrap();
    println!("printing tokens {}",tokens.len());
    for i in 0..tokens.len(){
        println!("{}: {:?}",i,tokens[i])
    }
    let mut parser = Parser::from_tokens(tokens);
    let program_node = parser.parse().unwrap();
    program_node.print_tree(0);
    print!("{}", program_node.getstring(0));
    fs::write("out.py", program_node.getstring(0)).unwrap();
}
