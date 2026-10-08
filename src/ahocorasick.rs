use std::collections::VecDeque;
#[derive(Debug)]
pub struct Ahocorasick{
    adj:Vec<Vec<Option<usize>>>,
    maxsuffix:Vec<usize>,
    flagword:Vec<Option<String>>,
    state:usize
}
const CHARARRLEN:usize = 40;
impl Ahocorasick{
    fn get_i(c:char)->Result<usize,String>{
        match c{
            'a'..='z' => {Ok(c as usize - 'a' as usize)},
            ' ' => Ok(26),
            '.' => Ok(27),
            ',' => Ok(28),
            '\n' => Ok(29),
            '0'..='9' => Ok(c as usize - '0' as usize + 30),
            _ => Err(format!("{} is not a valid character",c))
        }
    }
    fn get_c(i:usize)->Result<char,String>{
        match i{
            0..26=> Ok((i as u8 + 'a' as u8) as char),
            26 => Ok(' '),
            27 => Ok('.'),
            28 => Ok(','),
            29 => Ok('\n'),
            30..40 => Ok((i as u8 + '0' as u8) as char),
            _ => Err(format!("{} is out of bounds for character converstion",i))
        }
    }
    fn add_node(&mut self) -> usize{
        self.adj.push(vec![None;CHARARRLEN]);
        self.flagword.push(None);
        self.maxsuffix.push(0);
        return self.adj.len()-1;
    }
    pub fn add_word(&mut self, word:String)->Result<(),String>{
        let mut state = 0;
        for c in word.chars(){
            let ci = Ahocorasick::get_i(c)?;
            match self.adj[state][ci]{
                None => {
                    let next_state = self.add_node();
                    
                    self.adj[state][ci] = Some(next_state);
                    state = next_state;
                }
                Some(v) => state = v,
            }
        }
        self.flagword[state] = Some(word.clone());
        return Ok(());
    }
    fn get_max_suff(&self, word:&String)->Result<usize,String>{
        let chars:Vec<char> = word.chars().collect();
        let mut state = 0;
        for i in 1..chars.len(){
            state = self.next_state(state, chars[i])?;
        }
        return Ok(state);
    }
    fn next_state(&self, mut state:usize, c:char)->Result<usize,String>{
        let ci = Ahocorasick::get_i(c)?;

        while let None = self.adj[state][ci] && state != 0{
            state = self.maxsuffix[state];
        }
        match self.adj[state][ci]{
            None => Ok(0),
            Some(v) => Ok(v),
        }
    }
    pub fn new() -> Ahocorasick{
        return Ahocorasick { adj: vec![vec![None;CHARARRLEN];1], maxsuffix: vec![0;1], flagword: vec![None;1],state:0 }
    }
    pub fn from_words(keywords:Vec<String>) -> Result<Ahocorasick,String>{
        let mut corasick = Ahocorasick::new();
        for word in keywords{
            corasick.add_word(word)?;
        }
        let mut queue = VecDeque::new();
        queue.push_back((0,"".to_string()));
        while let Some((index, curstr)) = queue.pop_front(){
            corasick.maxsuffix[index] = corasick.get_max_suff(&curstr)?;
            for i in 0..corasick.adj[index].len(){
                if let Some(pos) = corasick.adj[index][i]{
                    let mut next_state_string = curstr.clone();
                    next_state_string.push(Ahocorasick::get_c(i)?);
                    queue.push_back((pos, next_state_string));
                }
            }
        }
        return Ok(corasick);
    }
    pub fn make_step(&mut self, c:char)->Result<Option<String>,String>{
        self.state = self.next_state(self.state, c)?;
        return Ok(self.flagword[self.state].clone());
    }
}