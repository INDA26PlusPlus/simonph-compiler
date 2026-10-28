use std::collections::{HashMap};
use super::tokanizer::Token;
use super::syntaxtree::*;
// struct Context{
//     functions:HashMap<String,u8>,
//     variables:HashSet<String>
// }

pub struct Parser{
    cur_function:Option<String>,
    functions:HashMap<String,u8>,
    tokens:Vec<Token>,
    i:usize,
}
impl Parser{
    pub fn from_tokens(tokens: Vec<Token>)->Parser{
        return Parser{functions:HashMap::new(),tokens,i:0,cur_function:None};
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
    fn parse_function_call(&mut self)->Result<Box<dyn Node>,String>{
        if let Token::The = self.get_token()? {} else {return Err("no the token".to_string())}
        let func_name = match self.get_token()?{
            Token::Text(a) => a,
            _ => return Err("no funcion identifier".to_string())
        };
        if !self.functions.contains_key(&func_name){
            return Err("textstring is not a function".to_string());
        }
        if let Token::Of = self.get_token().unwrap() {} else {panic!("no of token after function")}
        let parameters = self.parse_range(self.functions[&func_name]).unwrap();
        return Ok(Box::new(FunctionCallNode{func_name,parameters}));
    }
    fn parse_return_call(&mut self) -> Result<Box<dyn Node>,String>{
        if let Token::The = self.get_token()? {} else {return Err("no the token".to_string())}
        let func_name = match self.get_token()?{
            Token::Text(a) => a,
            _ => return Err("no funcion identifier".to_string())
        };
        let cur_function = match self.cur_function.as_ref(){
            None => return Err("not in function".to_string()),
            Some(v) => v,
        };
        if *cur_function != func_name{
            return Err("not the current function".to_string());
        }
        if let Token::Is = self.get_token()? {} else {return Err("no is token".to_string())};
        let set_to = self.parse_expression()?;
        return Ok(Box::new(ReturnCallNode{expression:set_to}));
    }
    fn parse_term(&mut self)->Result<Box<dyn Node>, String>{
        let reseti = self.i;
        if let Ok(node) = self.parse_number(){
            return Ok(node);
        }
        self.i = reseti;
        if let Ok(node) = self.parse_function_call(){
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
        let identifier_node = self.parse_identifier().unwrap();
        if let Token::To = self.get_token()? {} else{
            return Err("not a to token".to_string());
        }
        let expression_node = self.parse_expression().unwrap();
        return Ok(Box::new(SetNode{set_name:identifier_node, set_expression:expression_node}));

    }
    fn parse_print(&mut self)->Result<Box<dyn Node>,String>{
        if let Token::Print = self.get_token()? {} else{
            return Err("dident start with print".to_string());
        }
        let expression_node = self.parse_expression().unwrap();
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
    fn parse_func_identifier(&mut self) -> Result<Box<dyn Node>,String>{
        if let Ok(Token::A) = self.get_token(){}else{return Err("doesnt start with a".to_string())}
        if let Token::Text(v) = self.get_token()?{
            return Ok(Box::new(IdentifierNode{text:v}));
        }
        return Err("not a text token".to_string());
    }
    fn parse_func_range(&mut self) -> Result<(u8,Box<dyn Node>),String>{
        let node = self.parse_func_identifier()?;
        let reseti = self.i;
        if let Ok(Token::And) = self.get_token(){
            let (sz,continuation) = self.parse_func_range()?;
            return Ok((sz+1,Box::new(RangeNode{term:node,continuation:Some(continuation)})));
        }
        self.i = reseti;
        return Ok((1,Box::new(RangeNode{term:node,continuation:None})));
    }
    fn parse_func_decleration(&mut self)->Result<(Box<dyn Node>,(String,u8)),String>{
        if let Ok(Token::How) = self.get_token(){}else{return Err("doesnt start with how".to_string())}
        if let Ok(Token::To) = self.get_token(){}else{return Err("doesnt start with to".to_string())}
        if let Ok(Token::Get) = self.get_token(){}else{return Err("doesnt start with get".to_string())}
        if let Ok(Token::The) = self.get_token(){}else{return Err("doesnt start with the".to_string())}
        let func_name = match self.get_token()?{
            Token::Text(v) => v,
            _ => return Err("no function name".to_string())
        };
        if let Ok(Token::Of) = self.get_token(){}else{return Err("doesnt start with of".to_string())}
        let (range_sz, range_node) = self.parse_func_range()?;
        if let Ok(Token::Dot) = self.get_token(){}else{return Err("doesnt start with get".to_string())}
        if let Ok(Token::Newline) = self.get_token(){}else{return Err("doesnt start with the".to_string())}
        return Ok((Box::new(FunctionDeclerationNode{name:func_name.clone(),paramaters:range_node}),(func_name,range_sz)));
    }
    fn parse_func_block(&mut self)->Result<Box<dyn Node>,String>{
        let (funcdeclare,(func_name,func_len)) = self.parse_func_decleration()?;
        self.functions.insert(func_name.clone(), func_len);
        self.cur_function = Some(func_name);
        let code_block = self.parse_codeblock();
        self.cur_function = None;
        let fin_block = code_block.unwrap();
        return Ok(Box::new(FunctionBlockNode{decleration:funcdeclare,code_block:fin_block}));
    }
    fn parse_codeline(&mut self)->Result<Box<dyn Node>,String>{
        let reseti = self.i;
        println!("parsing line at {}",self.i);
        self.i = reseti;
        if let Ok(node) = self.parse_print(){
            if let Ok(Token::Dot) = self.get_token(){
                return Ok(node);
            }
            else{
                panic!("no dot at token {}", self.i-1);
            }
        }
        self.i = reseti;
        if let Ok(node) = self.parse_return_call(){
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
        self.i = reseti;
        if let Ok(node) = self.parse_if(){
            return Ok(node);
        }
        println!("failed to parse");
        return Err(format!("couldent parse a line {}",self.i));
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
    pub fn get_block(&mut self)->Result<Box<dyn Node>,String>{
        let reseti = self.i;
        if let Ok(node) = self.parse_func_block(){
            return Ok(node);
        }
        self.i = reseti;
        return self.parse_codeblock();
    }
    pub fn parse_program(&mut self)->Result<Box<dyn Node>,String>{
        println!("{}",self.i);
        let code_block = self.get_block().unwrap();
        println!("parsed block");
        match self.get_token().unwrap(){
            Token::Eof =>{
                return Ok(Box::new(ProgramNode{block:code_block, continuation:None}))
            }
            Token::Newline =>{
                let continuation = self.parse_program().unwrap();
                return Ok(Box::new(ProgramNode{block:code_block, continuation:Some(continuation)}))
            }
            _ => return Err(format!("no continuation at token {}", self.i-1))
        }
        
    }
    pub fn parse_binary_expression(&mut self)->Result<Box<dyn Node>,String>{
        let l = self.parse_expression()?;
        if let Token::Is = self.get_token()?{} else {return Err("no is after expression".to_string())}
        let expression = match self.get_token()?{
            Token::Equal => {
                if let Token::To = self.get_token()? {"=="} else {return Err("no to after equal".to_string())}
            },
            _ => return Err("no comparetor".to_string())
        }.to_string();
        let r = self.parse_expression().unwrap();
        return Ok(Box::new(BinaryExpressionNode{comparator:expression,left:l,right:r}))
    }
    pub fn parse_if(&mut self)->Result<Box<dyn Node>,String>{
        if let Token::If = self.get_token()?{} else {return Err("doesnt start with if".to_string())}
        let binop = self.parse_binary_expression().unwrap();
        if let Token::Then = self.get_token()?{} else {panic!("doesnt end with then")}
        if let Token::Newline = self.get_token()?{} else {panic!("doesnt end with newline")}
        let block = self.parse_codeblock().unwrap();
        return Ok(Box::new(IfNode{expression:binop,code_block:block}));
    }
    pub fn parse(&mut self)->Result<Box<dyn Node>,String>{
        return self.parse_program();
    }

}