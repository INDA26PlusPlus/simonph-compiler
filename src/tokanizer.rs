use super::ahocorasick::Ahocorasick;
use std::collections::HashMap;
#[derive(Debug,Clone)]
pub enum Token{
    Text(String),
    Number(i64),
    Dot,
    Comma,
    Set,
    Print,
    To,
    The,
    Newline,
    And,
    Sum,
    Difference,
    Product,
    Quotient,
    Of
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
            ("\nset ", Token::Set),
            (" print ", Token::Print),
            ("\nprint ", Token::Print),
            (" to ", Token::To),
            (" the ", Token::The),
            ("\n",Token::Newline),
            (" and ",Token::And),
            (" sum ", Token::Sum),
            (" of ", Token::Of),
            (" difference ", Token::Difference),
            (" product ", Token::Product),
            (" quotient ", Token::Quotient),
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
    pub fn get_token(chars: Vec<char>)->Token{
        let text:String = chars.iter().collect();
        match text.parse::<i64>(){
            Ok(v) => Token::Number(v),
            Err(_) =>  Token::Text(text),
        }
    }
    pub fn tokanize(&mut self, text:&str) -> Result<Vec<Token>,String>{
        let mut cur_chars:Vec<char> = Vec::new();
        let mut ret:Vec<Token> = Vec::new();
        self.ahocorasick.make_step('\n')?;
        for c in text.chars(){
            cur_chars.push(c);
            if let Some(flag) = self.ahocorasick.make_step(c)?{
                for _ in 0..flag.len(){
                    cur_chars.pop();
                }
                if !cur_chars.is_empty(){
                    ret.push(Tokanizer::get_token(cur_chars));
                    cur_chars = Vec::new();
                }
                ret.push(self.map[&flag].clone());
            }
        }
        if !cur_chars.is_empty(){
            ret.push(Tokanizer::get_token(cur_chars));
        }
        return Ok(ret);
    }
}