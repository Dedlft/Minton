#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    String(String),
    Int(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lparen,
    Rparen,
    Pipe,
    Semicolon,
    Identifier(String),
    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let ch = self.input[self.position];

        match ch {
            '|' => { self.position += 1; Token::Pipe }
            ';' => { self.position += 1; Token::Semicolon }
            '+' => { self.position += 1; Token::Plus }
            '-' => { self.position += 1; Token::Minus }
            '*' => { self.position += 1; Token::Asterisk }
            '/' => { self.position += 1; Token::Slash }
            '(' => { self.position += 1; Token::Lparen }
            ')' => { self.position += 1; Token::Rparen }
            '"' => self.read_string(),
            _ => {
                if ch.is_alphabetic() {
                    self.read_identifier()
                } else if ch.is_numeric(){
					self.read_number()
				} else {
                    self.position += 1;
                    Token::EOF
                }
            }
        }
    }
    
    fn read_number(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_numeric() {
            self.position += 1;
        }
        let content: String = self.input[start..self.position].iter().collect();
        Token::Int(content.parse().unwrap_or(0))
    }

    fn read_string(&mut self) -> Token {
        self.position += 1;
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position] != '"' {
            self.position += 1;
        }
        let content = self.input[start..self.position].iter().collect();
        self.position += 1;
        Token::String(content)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.position;
        while self.position < self.input.len() && self.input[self.position].is_alphanumeric() {
            self.position += 1;
        }
        let content = self.input[start..self.position].iter().collect();
        Token::Identifier(content)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }
    }
}
