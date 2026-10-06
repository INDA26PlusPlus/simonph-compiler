mod ahocorasick;
mod tokanizer;
use tokanizer::Tokanizer;
use tokanizer::Token;
fn main() {
    let mut tokanizer = Tokanizer::new().unwrap();
    println!("parsing tokens");
    let tokens = tokanizer.tokanize("set the first number to a.").unwrap();
    println!("printing tokens {}",tokens.len());
    for token in &tokens{
        println!("{:?}",token);
    }
}
