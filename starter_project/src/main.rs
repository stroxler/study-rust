use std::env::args;
use std::io::{BufRead, Write};
use std::rc::Rc;

pub fn main() {
    let args: Vec<String> = args().collect();
    match args.len() {
        1 => {
            match run_repl() {
                Ok(_) => {},
                Err(message) => {
                    println!("Encountered error: {:?}", message)
                },
            }
        },
        _ => {
            println!("Usage: cargo run");
        },
    }
}


fn prompt_and_flush() -> () {
    print!("calculator> ");
    let _ = std::io::stdout().flush();
}


fn run_repl() -> Result<(), String> {
    let stdin = std::io::stdin();
    prompt_and_flush();
    for line in stdin.lock().lines() {
        match line {
            Ok(code) => {
                match get_expression(code) {
                    Ok(expression) => println!("Expression: {:?}", expression),
                    Err(message) => println!("Syntax Error: {:?}", message),
                }
            },
            Err(err) => {
                println!("Error reading stdin: {:?}", err);
            }
        }
        prompt_and_flush();
    }
    Ok(())
}


fn get_expression(code: String) -> Result<Rc<Expression>, String> {
    let tokens = lex_code(code)?;
    let expression = parse_code(tokens)?;
    Ok(expression)
}


/*** LEXING ***/

fn lex_code(code: String) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(code);
    lexer.lex()?;
    Ok(lexer.tokens)
}


pub struct Lexer {
    chars: Vec<char>,
    start: usize,
    current: usize,
    tokens: Vec<Token>
}

impl Lexer {

    pub fn new(code: String) -> Lexer {
        Lexer {
            chars: code.chars().collect(),
            start: 0,
            current: 0,
            tokens: Vec::new()
        }
    }

    pub fn lex(&mut self) -> Result<(), String> {
        while !self.reached_end() {
            self.scan_token()?;
        }
        self.add_token(Token::eof(self.start));
        Ok(())
    }

    // scanning functions

    pub fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_symbol(TokenKind::LeftParen, c),
            ')' => self.add_symbol(TokenKind::RightParen, c),
            '+' => self.add_symbol(TokenKind::Plus, c),
            '-' => self.add_symbol(TokenKind::Minus, c),
            '*' => self.add_symbol(TokenKind::Star, c),
            '0'..='9' => self.scan_number(c),
            ' ' => self.scan_token(),
            _ => Err(format!("Unexpected character '{}' at position {}", c, self.current))
        }
    }

    pub fn scan_number(&mut self, c: char) -> Result<(), String> {
        let before_decimal = self.scan_digits(vec![c]);
        let after_decimal = match self.peek() {
            Some('.') => {
                self.consume();
                self.scan_digits(vec![])
            }
            _ => {
                vec![]
            }
        };
        let number = Lexer::create_number(before_decimal, after_decimal);
        let lexeme = self.get_lexeme();
        self.add_number(number, lexeme)
    }

    fn scan_digits(&mut self, mut chars: Vec<char>) -> Vec<char> {
        let mut stop = false;
        while !stop {
            match self.peek() {
                Some(c) if c.is_digit(10) => {
                        self.consume();
                        chars.push(c);
                    }
                _ => {
                    stop = true;
                }
            }
        };
        chars
    }

    fn create_number(before_decimal: Vec<char>, after_decimal: Vec<char>) -> f64 {
        let floor: f64 = before_decimal.into_iter().collect::<String>().parse().unwrap();
        let n_decimal_digits = after_decimal.len();
        if n_decimal_digits > 0 {
            let after_decimal_int: i64 = after_decimal.into_iter().collect::<String>().parse().unwrap();
            floor + ((after_decimal_int as f64) / 10_f64.powf(n_decimal_digits as f64))
        } else {
            floor
        }
    }



    pub fn add_symbol(&mut self, kind: TokenKind, c: char) -> Result<(), String> {
        let token = Token {
            kind,
            lexeme: vec![c].into_iter().collect(),
            start: self.start,
            literal: None,
        };
        Ok(self.add_token(token))
    }

    pub fn add_number(&mut self, number: f64, lexeme: String) -> Result<(), String> {
        let token = Token {
            kind: TokenKind::Number,
            lexeme,
            start: self.start,
            literal: Some(number),
        };
        Ok(self.add_token(token))
    }

    pub fn add_token(&mut self, token: Token) -> () {
        self.tokens.push(token);
        self.start = self.current;
    }

    // low-level character utilities

    pub fn reached_end(&self) -> bool {
        self.chars.len() == self.current
    }

    fn advance(&mut self) -> char {
        let c = *self.chars.get(self.current).unwrap();
        self.current += 1;
        c
    }

    fn peek(&mut self) -> Option<char> {
        self.chars.get(self.current).copied()
    }

    fn consume(&mut self) -> () {
        self.current += 1;
    }

    fn get_lexeme(&self) -> String {
        self.chars[self.start..(self.current)].iter().collect()
    }

}


#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    pub literal: Option<f64>,
    pub start: usize,
}

impl Token {
    pub fn eof(start: usize) -> Token {
        Token {
            kind: TokenKind::Eof,
            lexeme: "".to_string(),
            literal: None,
            start
        }
    }

    pub fn symbol(kind: TokenKind, lexeme: String, start: usize) -> Token {
        Token {
            kind,
            lexeme,
            literal: None,
            start
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenKind {
    Number,
    Plus,
    Minus,
    Star,
    LeftParen,
    RightParen,
    Eof,
}

/*** PARSING ***/

fn parse_code(tokens: Vec<Token>) -> Result<Rc<Expression>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}


pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Rc<Expression>, String> {
        self.term()
    }

    fn term(&mut self) -> Result<Rc<Expression>, String> {
        let mut expression = self.product()?;
        let mut finished = false;
        while !finished {
            let tk = self.peek().kind;
            match tk {
                TokenKind::Plus => {
                    self.consume();
                    let right = self.product()?;
                    expression = Rc::new(Expression::Sum(expression, right));
                },
                TokenKind::Minus => {
                    self.consume();
                    let right = self.product()?;
                    expression = Rc::new(Expression::Difference(expression, right));
                },
                _ => finished = true,
            }
        }
        Ok(expression)
    }

    fn product(&mut self) -> Result<Rc<Expression>, String> {
        let mut expression = self.parenthesized()?;
        let mut finished = false;
        while !finished {
            let tk = self.peek().kind;
            match tk {
                TokenKind::Star => {
                    self.consume();
                    let right = self.parenthesized()?;
                    expression = Rc::new(Expression::Product(expression, right));
                },
                _ => finished = true,
            }
        };
        Ok(expression)
    }

    fn parenthesized(&mut self) -> Result<Rc<Expression>, String> {
        let t = self.peek();
        match t.kind {
            TokenKind::LeftParen => {
                self.consume();
                let inner = self.parse()?;
                let t = self.advance();
                match t.kind {
                    TokenKind::Eof => Err(format!("Unexpected end of input: {:?}", t)),
                    TokenKind::RightParen => Ok(inner),
                    _ => Err(format!("Unexpected token: {:?}", t)),
                }
            },
            _ => self.number()
        }
    }

    fn number(&mut self) -> Result<Rc<Expression>, String> {
        let t = self.advance();
        match t.kind {
            TokenKind::Eof => Err(format!("Unexpected end of input: {:?}", t)),
            TokenKind::Number => {
                if let Some(number) = t.literal {
                    Ok(Rc::new(Expression::Number(number)))
                } else {
                    Err(format!("Unexpected number token with no number {:?} (lexer bug!)", t))
                }
            },
            _ => Err(format!("Could not parse token: {:?}", t)),
        }

    }

    fn advance(&mut self) -> &Token {
        let t = self.tokens.get(self.current).unwrap();
        self.current += 1;
        t
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn consume(&mut self) -> () {
        self.current += 1;
        if self.current >= self.tokens.len() {
            panic!("We consumed the last token! Missed a check for Eof");
        }
    }

}



#[derive(Debug)]
pub enum Expression {
    Number(f64),
    Sum(Rc<Expression>, Rc<Expression>),
    Difference(Rc<Expression>, Rc<Expression>),
    Product(Rc<Expression>, Rc<Expression>),
}
