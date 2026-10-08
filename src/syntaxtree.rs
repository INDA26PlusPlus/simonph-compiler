

//trait for nodes
pub trait Node {
    fn getstring(&self) -> String;
    fn print_tree(&self, nr_tabs:u8);
}

pub struct IdentifierNode{
    pub text:String,
}

impl Node for IdentifierNode{
    fn getstring(&self) -> String{
        return self.text.replace(' ', "_");
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str(&self.getstring());
        println!("{ret}");
    }
}

pub struct NumberNode{
    pub value:i64,
}

impl Node for NumberNode{
    fn getstring(&self) -> String {
        return self.value.to_string();
    }
    fn print_tree(&self, nr_tabs:u8) {
        let mut ret = "".to_string();
        for _ in 0..nr_tabs{
            ret.push('\t');
        }
        ret.push_str(&self.getstring());
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
    fn getstring(&self) -> String {
        let mut ret = "print(".to_string();
        ret.push_str(&self.expression.getstring());
        ret.push(')');
        return ret;
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
    fn getstring(&self) -> String {
        let mut ret = "".to_string();
        ret.push_str(&self.set_name.getstring());
        ret.push_str("=");
        ret.push_str(&self.set_expression.getstring());
        return ret;
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
    fn getstring(&self) -> String {
        let mut ret = self.term.getstring();
        if let Some(v) = self.continuation.as_ref(){
            ret.push_str("and");
            ret.push_str(&v.getstring());
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
    fn getstring(&self) -> String {
        let mut ret = "(".to_string();
        ret.push_str(&self.term1.getstring());
        ret.push(self.operation_char);
        ret.push_str(&self.term2.getstring());
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
    fn getstring(&self) -> String {
        let mut ret = "".to_string();
        for _ in 0..self.nrtabs{
            ret.push('\t');
        }
        ret.push_str(&self.line_node.getstring());
        ret.push('\n');
        if let Some(next_block) = self.next_block.as_ref(){
            ret.push_str(&next_block.getstring());
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