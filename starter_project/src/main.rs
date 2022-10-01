use std::env::args;
use std::io::{BufRead, Write};

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
                let ast = get_ast(code)?;
                println!("AST: {:?}", ast);
            },
            Err(_) => {
                break;
            }
        }
        prompt_and_flush();
    }
    Ok(())
}


fn get_ast(code: String) -> Result<Ast, String> {
    let tokens = lex_code(code)?;
    let ast = parse_code(tokens)?;
    Ok(ast)
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
            '*' => self.add_symbol(TokenKind::Multiply, c),
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

#[derive(Debug)]
pub enum TokenKind {
    Number,
    Identifier,
    Assign,
    Plus,
    Minus,
    Multiply,
    LeftParen,
    RightParen,
    Eof,
}

/*** PARSING ***/

fn parse_code(tokens: Vec<Token>) -> Result<Ast, String> {
    Ok(Ast {tokens})
}



#[derive(Debug)]
pub struct Ast {
    tokens: Vec<Token>
}
