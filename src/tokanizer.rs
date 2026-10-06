use super::ahocorasick::Ahocorasick;
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub enum Token{
    Text(String),
    Dot,
    Comma,
    Set,
    Print,
    To
}
#[derive(Debug)]
pub struct Tokanizer{
    ahocorasick:Ahocorasick,
    map:HashMap<String,Token>
}
impl Tokanizer{
    pub fn new()->Result<Tokanizer,String>{
        let string_token_pairs = [
            (".", Token::Dot),
            (",",Token::Comma),
            (" set ", Token::Set),
            (" print ", Token::Print),
            (" to ", Token::To)
        ];
        let mut map: HashMap<String,Token> = HashMap::new();
        for (key, token) in string_token_pairs.clone(){
            map.insert(key.to_string(), token);
        }
        let mut words = Vec::new();
        for (x,_) in string_token_pairs{
            words.push(x.to_string());
        }
        return Ok(Tokanizer{ahocorasick:Ahocorasick::from_words(words)?, map});
    }
    pub fn tokanize(&mut self, text:&str) -> Result<Vec<Token>,String>{
        let mut cur_chars:Vec<char> = Vec::new();
        let mut ret:Vec<Token> = Vec::new();
        for c in text.chars(){
            cur_chars.push(c);
            if let Some(flag) = self.ahocorasick.make_step(c)?{
                for _ in 0..flag.len(){
                    cur_chars.pop();
                }
                if !cur_chars.is_empty(){
                    ret.push(Token::Text(cur_chars.into_iter().collect()));
                    cur_chars = Vec::new();
                }
                ret.push(self.map[&flag].clone());
            }
        }
        if !cur_chars.is_empty(){
            ret.push(Token::Text(cur_chars.into_iter().collect()));
        }
        return Ok(ret);
    }
}