use std::*;

/*#[derive(Debug)]
enum AST {
    HEAD(Vec<AST>),
    VALUE(String), // think i change this
    ASSIGN(Box<AST>, Box<AST>),
    FUNCIONCALL(String, Box<AST>),
    EXPR(Vec<AST>),
    RETURN(Box<AST>),
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut main_expr = Vec::new();
        let ast = AST::HEAD(main_expr);

        while self.pos < self.tokens.len() {
            let token = &self.tokens[self.pos];

            match token {
                Token::LPAREN => {}
                Token::RPAREN => {}
                Token::LBRACE => {}
                Token::RBRACE => {}
                Token::COMMA => {}
                Token::LANGLE => {}
                Token::RANGLE => {}
                Token::COLON => {}
                Token::WORD(name) => {
                    main_expr.push(AST::FUNCIONCALL(name.clone(), Box::new(self.parse_expr())));
                }
                Token::SEMICOLON => {}
                Token::NUMBER(_) => {}
            }

            self.pos += 1;
        }

        ast
    }
}*/

#[derive(Debug)]
enum TokenType {
    LParen,
    RParen,
    LBrace,
    RBrace,
    Comma,
    LAngle,
    RAngle,
    Colon,
    Identifier,
    Semicolon,
    Number
}

#[derive(Debug)]
struct Token {
    token_type: TokenType,
    value: String,
    char_pos: i32,
    line: i32
}

impl Token {
    pub fn new(token_type: TokenType, value: String, char_pos: i32, line: i32) -> Token {
        Token {
            token_type,
            value,
            char_pos,
            line
        }
    }
}

struct Lexer {}

impl Lexer {
    pub fn tokenize(str: String) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut current_word = "".to_string();
        let mut current_number = "".to_string();

        let mut char_pos = 0;
        let mut line = 1;

        for c in str.chars() {
            match c {
                '(' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::LParen, c.to_string(), char_pos, line));
                },
                ')' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::RParen, c.to_string(), char_pos, line));
                },
                ':' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::Colon, c.to_string(), char_pos, line));
                },
                ';' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::Semicolon, c.to_string(), char_pos, line));
                },
                ',' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::Comma, c.to_string(), char_pos, line));
                },
                '<' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::LAngle, c.to_string(), char_pos, line));
                },
                '>' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::RAngle, c.to_string(), char_pos, line));
                },
                '{' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::LBrace, c.to_string(), char_pos, line));
                },
                '}' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, char_pos);
                    tokens.push(Token::new(TokenType::RBrace, c.to_string(), char_pos, line));
                },
                _ => {
                    if c.is_numeric() {
                        if  current_word.len() != 0 {
                            current_word.push(c);
                        } else {
                            current_number.push(c);
                        }

                        continue;
                    }

                    if c.is_alphabetic() {
                        current_word.push(c);
                    } else {
                        if Self::is_token(c) {
                            if current_word.len() > 0 && !c.is_numeric() {
                                tokens.push(Token::new(TokenType::Identifier, current_word, char_pos, line));
                                current_word = "".to_string();
                            }
                            if current_number.len() > 0 {
                                tokens.push(Token::new(TokenType::Number, current_number, char_pos, line));
                                current_number = "".to_string();
                            }
                        } else {
                            eprintln!("Invalid character: {}", c);
                        }
                    }

                    if c == '\n' {
                        line += 1;
                        char_pos = -1;
                    }
                }
            }

            char_pos += 1;
        }

        tokens
    }

    fn check_word_number_end(vec: &mut Vec<Token>, word: &mut String, number: &mut String, line: i32, char_pos: i32) {
        if word.len() > 0 {
            vec.push(Token::new(TokenType::Identifier, word.clone(), char_pos, line));
            word.clear();
        }
        if number.len() > 0 {
            vec.push(Token::new(TokenType::Number, number.clone(), char_pos, line));
            number.clear();
        }
    }

    fn is_token(c: char) -> bool {
        match c {
            '(' => true,
            ')' => true,
            ':' => true,
            ';' => true,
            ',' => true,
            '<' => true,
            '>' => true,
            '{' => true,
            '}' => true,
            _ => {
                if c.is_numeric() {
                    true
                } else {
                    if c.is_alphabetic() || c.is_whitespace() {
                        true
                    } else {
                        false
                    }
                }
            }
        }
    }
}

fn main() {
    let intput = r#"
    main: func(args: Array<string>): int32 {
        return 0;
    }
"#;
    println!("Input {}", intput);
    let tokens = Lexer::tokenize(intput.to_string());
    println!("Tokens: {:?}", tokens);
}
