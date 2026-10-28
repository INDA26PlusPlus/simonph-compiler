

//trait for nodes
pub trait Node {
    fn getstring(&self,nr_tabs:u8) -> String;
    fn print_tree(&self, nr_tabs:u8);
}
fn get_indent(input:String, nr_tabs:u8)->String{
    let mut ret = "".to_string();
    for _ in 0..nr_tabs{
        ret.push('\t');
    }
    ret.push_str(&input);
    return ret;
}
pub struct IdentifierNode{
    pub text:String,
}

impl Node for IdentifierNode{
    fn getstring(&self,_:u8) -> String{
        return self.text.replace(' ', "_");
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str(&self.getstring(0));
        println!("{ret}");
    }
}

pub struct NumberNode{
    pub value:i64,
}

impl Node for NumberNode{
    fn getstring(&self,_:u8) -> String {
        return self.value.to_string();
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str(&self.getstring(0));
        println!("{ret}");
    }
}

pub struct SetNode{
    pub set_name:Box<dyn Node>,
    pub set_expression:Box<dyn Node>,
}

pub struct PrintNode{
    pub expression: Box<dyn Node>,
}
impl Node for PrintNode{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "print(".to_string();
        ret.push_str(&self.expression.getstring(0));
        ret.push(')');
        return get_indent(ret,nr_tabs);
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("print");
        println!("{ret}");
        self.expression.print_tree(nr_tabs +1);
    }
}

impl Node for SetNode{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "".to_string();
        ret.push_str(&self.set_name.getstring(0));
        ret.push_str("=");
        ret.push_str(&self.set_expression.getstring(0));
        return get_indent(ret, nr_tabs);
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("set");
        println!("{ret}");
        self.set_name.print_tree(nr_tabs +1);
        self.set_expression.print_tree(nr_tabs+1);
    }
}

pub struct RangeNode{
    pub term:Box<dyn Node>,
    pub continuation:Option<Box<dyn Node>>
}
impl Node for RangeNode{
    fn getstring(&self,_:u8) -> String {
        let mut ret = self.term.getstring(0);
        if let Some(v) = self.continuation.as_ref(){
            ret.push_str(",");
            ret.push_str(&v.getstring(0));
        }
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("range");
        println!("{ret}");
        self.term.print_tree(nr_tabs +1);
        if let Some(node) = self.continuation.as_ref(){
            node.print_tree(nr_tabs+1);
        }
    }
}
pub struct AlgebraNode{
    pub operation_char:char,
    pub term1:Box<dyn Node>,
    pub term2: Box<dyn Node>,
}
impl Node for AlgebraNode{
    fn getstring(&self,_:u8) -> String {
        let mut ret = "(".to_string();
        ret.push_str(&self.term1.getstring(0));
        ret.push(self.operation_char);
        ret.push_str(&self.term2.getstring(0));
        ret.push(')');
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("arithmetic ");
        ret.push(self.operation_char);
        println!("{ret}");
        self.term1.print_tree(nr_tabs +1);
        self.term2.print_tree(nr_tabs+1);
    }
    
}
pub struct CodeBlock{
    pub nrtabs:u8,
    pub next_block: Option<Box<dyn Node>>,
    pub line_node: Box<dyn Node>,
}
impl Node for CodeBlock{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "".to_string();
        for _ in 0..self.nrtabs{
            ret.push('\t');
        }
        ret.push_str(&self.line_node.getstring(nr_tabs));
        ret.push('\n');
        if let Some(next_block) = self.next_block.as_ref(){
            ret.push_str(&next_block.getstring(nr_tabs));
        }
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("codeblock");
        println!("{ret}");
        self.line_node.print_tree(nr_tabs +1);
        if let Some(node) = self.next_block.as_ref(){
            node.print_tree(nr_tabs+1);
        }
    }
}
pub struct FunctionDeclerationNode{
    pub name:String,
    pub paramaters:Box<dyn Node>
}
impl Node for FunctionDeclerationNode{
    fn getstring(&self,_nr_tabs:u8) -> String {
        let mut ret = "".to_string();
        ret.push_str(&self.name);
        ret.push_str("(");
        ret.push_str(&self.paramaters.getstring(0));
        ret.push_str("):");
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        println!("function decleration for {}",self.name);
        println!("paramaters");
        self.paramaters.print_tree(nr_tabs+1);
    }
}
pub struct FunctionBlockNode{
    pub decleration:Box<dyn Node>,
    pub code_block: Box<dyn Node>
}
impl Node for FunctionBlockNode{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "".to_string();
        ret.push_str("@cache\n");
        ret.push_str("def ");
        ret.push_str(&self.decleration.getstring(nr_tabs));
        ret.push('\n');
        ret.push_str(&self.code_block.getstring(nr_tabs+1));
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        println!("decleration");
        self.decleration.print_tree(nr_tabs+1);
        println!("code block");
        self.code_block.print_tree(nr_tabs+1);
    }
}
pub struct ProgramNode{
    pub block:Box<dyn Node>,
    pub continuation:Option<Box<dyn Node>>
}
impl Node for ProgramNode{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "".to_string();
        ret.push_str(&self.block.getstring(nr_tabs));
        ret.push_str("\n");
        if let Some(node) = self.continuation.as_ref(){
            ret.push_str(&node.getstring(nr_tabs));
        }
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("block");
        println!("{ret}");
        self.block.print_tree(nr_tabs+1);
        if let Some(continuation) = self.continuation.as_ref(){
            continuation.print_tree(nr_tabs);
        }
    }
}
pub struct FunctionCallNode{
    pub func_name:String,
    pub parameters:Box<dyn Node>
}
impl Node for FunctionCallNode{
    fn getstring(&self,_:u8) -> String {
        let mut ret = self.func_name.clone();
        ret.push('(');
        ret.push_str(&self.parameters.getstring(0));
        ret.push(')');
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("function call for ");
        ret.push_str(&self.func_name);
        println!("{ret}");
        self.parameters.print_tree(nr_tabs+1);
    }
}
pub struct ReturnCallNode{
    pub expression:Box<dyn Node>
}
impl Node for ReturnCallNode{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "return ".to_string();
        ret.push_str(&self.expression.getstring(0));
        return get_indent(ret,nr_tabs);
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str("return");
        println!("{ret}");
        self.expression.print_tree(nr_tabs+1);
    }
}

pub struct BinaryExpressionNode{
    pub comparator:String,
    pub left:Box<dyn Node>,
    pub right:Box<dyn Node>
}
impl Node for BinaryExpressionNode{
    fn getstring(&self,_:u8) -> String {
        let mut ret = "".to_string();
        ret.push_str(&self.left.getstring(0));
        ret.push_str(" ");
        ret.push_str(&self.comparator);
        ret.push_str(" ");
        ret.push_str(&self.right.getstring(0));
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        println!("{}",get_indent(format!("binary operator {}", self.comparator), nr_tabs));
        self.left.print_tree(nr_tabs+1);
        self.right.print_tree(nr_tabs+1);
    }
}
pub struct IfNode{

    pub expression: Box<dyn Node>,
    pub code_block: Box<dyn Node>
}
impl Node for IfNode{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "".to_string();
        ret.push_str("if ");
        ret.push_str(&self.expression.getstring(nr_tabs));
        ret.push_str(":\n");
        ret = get_indent(ret, nr_tabs);
        ret.push_str(&self.code_block.getstring(nr_tabs+1));
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        println!("{}",get_indent("if".to_string(), nr_tabs));
        println!("{}", get_indent("expression".to_string(),nr_tabs));
        self.expression.print_tree(nr_tabs+1);
        println!("{}", get_indent("code block".to_string(), nr_tabs));
        self.code_block.print_tree(nr_tabs+1);
    }
}
pub struct RootNode{
    pub program_node:Box<dyn Node>
}
impl Node for RootNode{
    fn getstring(&self,nr_tabs:u8) -> String {
        let mut ret = "".to_string();
        ret.push_str("from functools import cache\n");
        ret.push_str(&self.program_node.getstring(nr_tabs));
        return ret;
    }
    fn print_tree(&self, nr_tabs:u8) {
        self.program_node.print_tree(nr_tabs);
    }
}