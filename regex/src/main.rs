
mod Lex {

    #[derive(Debug)]
    pub enum TokenType {
        Character(char),
        LeftParen,
        RightParen,
        Pipe,
        Dot,
        Star,
        Eof,
        // I'm not implementing features that can easily
        // be achieved via preprocessing such as: +, ?, ranges
    }

    /* Normally a token would include start and end positions and the actual
     * lexeme. But the regular expression grammar is so simple (each token is
     * one or two characters) that we can get away without this, which simplifies
     * the code a bit
     */
    #[derive(Debug)]
    pub struct Token {
        position: usize,
        tt: TokenType,
    }

    pub struct Lexer {
        chars: Vec<char>,
        current: usize,
        tokens: Vec<Token>,
    }

    impl Lexer {
        pub fn new(code: String) -> Lexer {
            Lexer {
                chars: code.chars().collect(),
                current: 0,
                tokens: Vec::new(),
            }
        }

        fn reached_end(&self) -> bool {
            self.current >= self.chars.len()
        }


        fn peek(&self) -> Option<char> {
            self.chars.get(self.current).map(|c| c.clone())
        }

        fn consume(&mut self) {
            self.current += 1
        }

        fn advance(&mut self) -> char {
            let c = self.peek().unwrap();
            self.consume();
            c
        }

        fn add_token(&mut self, tt: TokenType) -> () {
            self.tokens.push(
                Token {
                    position: self.current,
                    tt,
                }
            )
        }

        pub fn lex(&mut self) -> Result<(), String> {
            while (!self.reached_end()) {
                let c = self.advance();
                match c {
                    '\\' => {
                        if let Some(c1) = self.peek() {
                            self.consume();
                            self.add_token(
                                TokenType::Character(c1)
                            )
                        } else {
                            return Err("Unexpected end of input after '\\`'".to_string())
                        }
                    },
                    '.' => {
                        self.add_token(TokenType::Dot)
                    },
                    '*' => {
                        self.add_token(TokenType::Star)
                    },
                    '|' => {
                        self.add_token(TokenType::Pipe)
                    }
                    '(' => {
                        self.add_token(TokenType::LeftParen)
                    },
                    ')' => {
                        self.add_token(TokenType::RightParen)
                    },
                    _ => {
                        self.add_token(
                            TokenType::Character(c)
                        )
                    },
                }

            }
            self.add_token(TokenType::Eof);
            Ok(())
        }
    }

    pub fn lex(code: String) -> Result<Vec<Token>, String> {
        let mut lexer = Lexer::new(code);
        lexer.lex()?;
        Ok(lexer.tokens)
    }

}


fn main() {
    let examples = vec![
        // Good examples
        r"a",
        r"a*.(bc|d)",
        r"(\\\(\|\.\a\b\))",
        // Bad examples
        r"a\",
    ];
    for raw in examples {
        let tokens = Lex::lex(raw.to_string());
        println!("'{:}' -> {:?}", raw, tokens)

    }
}
