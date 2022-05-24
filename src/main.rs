use std::env; // For parsing arguments
use std::fs; // For file system

fn main() {
    let file = env::args().nth(1).unwrap();

    let contents = fs::read_to_string(file).unwrap();

    let mut lexer = Lexer::new(contents);
    lexer.lex();
}



// Tokens
#[derive(Debug)]
enum TokenKind {
    Identifier,
    Declare, Separator,
    Str, Integer, Real, Boolean,

}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    literal: String,
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self { kind, literal }
    }
}


// Lexer
#[derive(Debug)] // For better printing methods
struct Lexer {
    source: Vec<char>, 
    counter: usize,
}

impl Lexer {
    
    pub fn new(new_content: String) -> Self{
        Self { source: new_content.chars().collect(), counter: 0 }
    }

    pub fn lex(&mut self) {
        let mut tokens:Vec<Token> = Vec::new();

        while self.source.len() > self.counter {
            let c = self.current_char();

            match c { 
                ':' => {
                    tokens.push(Token::new(TokenKind::Separator, ":".to_owned()));
                    self.counter += 1
                },

                // Strings
                '\"' => {
                    self.counter += 1;
                    let mut buffer = String::new();
                    while self.current_char() != '\"' {
                        buffer.push(self.current_char()); // Push the characters to the String buffer

                        self.counter += 1;
                    }
                   

                    tokens.push(Token::new(TokenKind::Str, buffer));
                    self.counter += 1;

                },

                // "_" represents any other character not mentioned specifically

                _ if self.current_char().is_numeric() => { 
                    let mut buffer = String::new();
                    while self.current_char().is_numeric() {
                        buffer.push(self.current_char()); // Push the characters to the String buffer

                        self.counter += 1;
                    }
                   

                    tokens.push(Token::new(TokenKind::Integer, buffer));
                    self.counter += 1;


                }

                
                _ if self.current_char().is_alphabetic() => { 
                    let mut buffer = String::new();
                    while self.current_char().is_alphabetic() {
                        buffer.push(self.current_char());
                        self.counter += 1;
                    }

                    // Check if its a keyword
                    let kind: TokenKind = match buffer.as_str() {
                        "DECLARE" => TokenKind::Declare,
                        "STRING" => TokenKind::Str,
                        "INTEGER" => TokenKind::Integer,
                        "REAL" => TokenKind::Real,
                        "BOOLEAN" => TokenKind::Boolean,
                        _ => TokenKind::Identifier, // Assuming it is an identifier
                    };

                    tokens.push(Token::new(kind, buffer)); 
                }


                _ => {
                    self.counter += 1
                }
            }

        }
        println!("{:?}", tokens);
    }

    // Get current Character
    fn current_char(&self) -> char {
        *self.source.get(self.counter).unwrap()
    }
}

