use std::collections::{HashMap, HashSet};
use super::tokanizer::Token;
use super::syntaxtree::*;
// struct Context{
//     functions:HashMap<String,u8>,
//     variables:HashSet<String>
// }

pub struct Parser{
    //context:Context,
    tokens:Vec<Token>,
    i:usize,
}
impl Parser{
    pub fn from_tokens(tokens: Vec<Token>)->Parser{
        return Parser{tokens,i:0};
    }
    // fn new()->Parser{
    //     return Parser{context:Context { functions: HashMap::new(), variables: HashSet::new() }}
    // }
    fn get_token(&mut self)->Result<Token,String>{
        if self.i >= self.tokens.len(){
            return Err("Index out of bounds".to_string());
        }
        let token = &self.tokens[self.i];
        self.i += 1;
        return Ok(token.clone());
    }
    fn parse_identifier(&mut self)->Result<Box<dyn Node>,String>{
        if let Token::The = self.get_token()? {} else{
            return Err("no the before identifier".to_string());
        }
        if let Token::Text(v) = self.get_token()?{
            return Ok(Box::new(IdentifierNode{text:v}));
        }
        return Err("not a text token".to_string());
    }
    fn parse_number(&mut self)->Result<Box<dyn Node>,String>{
        if let Token::Number(v) = self.get_token()?{
            return Ok(Box::new(NumberNode{value:v}));
        }
        return Err("not a text token".to_string());
    }
    fn parse_term(&mut self)->Result<Box<dyn Node>, String>{
        let reseti = self.i;
        if let Ok(node) = self.parse_number(){
            return Ok(node);
        }
        self.i = reseti;
        return self.parse_identifier();
    }
    fn parse_algebra(&mut self)->Result<Box<dyn Node>,String>{
        if let Token::The = self.get_token()? {} else {return Err("no the token".to_string())}
        let operation_char = match self.get_token()?{
            Token::Sum => '+',
            Token::Difference => '-',
            Token::Product => '*',
            Token::Quotient => '/',
            _ => return Err("not a valid algebra token".to_string()),
        };
        if let Token::Of = self.get_token()? {} else {return Err("no of token".to_string())}
        let term1 = self.parse_expression()?;
        if let Token::And = self.get_token()? {} else {return Err("no and token".to_string())}
        let term2 = self.parse_expression()?;
        return Ok(Box::new(AlgebraNode{operation_char,term1,term2}))
    }
    fn parse_expression(&mut self)->Result<Box<dyn Node>,String>{
        let reseti = self.i;
        if let Ok(node) = self.parse_term(){
            return Ok(node);
        }
        self.i = reseti;
        if let Ok(node) = self.parse_algebra(){
            return Ok(node);
        }
        return Err("couldent parse expression".to_string());
    }
    fn parse_set(&mut self)->Result<Box<dyn Node>,String>{
        if let Token::Set = self.get_token()? {} else{
            return Err("dident start with set".to_string());
        }
        let identifier_node = self.parse_identifier()?;
        if let Token::To = self.get_token()? {} else{
            return Err("not a to token".to_string());
        }
        let expression_node = self.parse_expression()?;
        return Ok(Box::new(SetNode{set_name:identifier_node, set_expression:expression_node}));

    }
    fn parse_print(&mut self)->Result<Box<dyn Node>,String>{

        if let Token::Print = self.get_token()? {} else{
            return Err("dident start with print".to_string());
        }
        let expression_node = self.parse_expression()?;
        return Ok(Box::new(PrintNode{expression:expression_node}));
    }
    fn parse_range(&mut self, nr_elemts:u8)->Result<Box<dyn Node>,String>{
        let node = self.parse_expression()?;
        if nr_elemts == 1{
            return Ok(Box::new(RangeNode{term:node,continuation:None}));
        }
        if let Token::And = self.get_token()? {} else{
            return Err("No and inbetween range elements".to_string());
        }
        let node2 = self.parse_range(nr_elemts-1)?;
        return Ok(Box::new(RangeNode{term:node,continuation:Some(node2)}));
    }
    fn parse_codeline(&mut self)->Result<Box<dyn Node>,String>{
        let reseti = self.i;
        if let Ok(node) = self.parse_print(){
            if let Ok(Token::Dot) = self.get_token(){
                return Ok(node);
            }
        }
        self.i = reseti;
        if let Ok(node) = self.parse_set(){
            if let Ok(Token::Dot) = self.get_token(){
                return Ok(node);
            }
        }
        return Err("couldent parse a line".to_string());
    }
    fn parse_codeblock(&mut self)->Result<Box<dyn Node>,String>{
        let node = self.parse_codeline()?;
        let mut reseti = self.i;
        if let Ok(node2) = self.parse_codeblock(){
            return Ok(Box::new(CodeBlock{nrtabs:0,next_block:Some(node2),line_node:node}));
        }
        self.i = reseti;
        if let Token::Newline = self.get_token()?{
            reseti = self.i;
            if let Ok(node2) = self.parse_codeblock(){
                return Ok(Box::new(CodeBlock{nrtabs:0,next_block:Some(node2),line_node:node}));
            }
            self.i = reseti;
            return Ok(Box::new(CodeBlock{nrtabs:0,next_block:None,line_node:node}));
        }
        return Err("couldent parse block".to_string());
    }
    pub fn parse_program(&mut self)->Result<Box<dyn Node>,String>{
        return self.parse_codeblock();
    }

}