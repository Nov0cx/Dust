#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub char_pos: i32,
    pub line: i32
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

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(str: String) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut current_word = "".to_string();
        let mut word_start_pos = 0;
        let mut current_number = "".to_string();
        let mut number_start_pos = 0;

        let mut char_pos = 1;
        let mut line = 1;

        for c in str.chars() {
            match c {
                '(' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::LParen, c.to_string(), char_pos, line));
                },
                ')' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::RParen, c.to_string(), char_pos, line));
                },
                ':' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::Colon, c.to_string(), char_pos, line));
                },
                ';' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::Semicolon, c.to_string(), char_pos, line));
                },
                ',' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::Comma, c.to_string(), char_pos, line));
                },
                '<' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::LAngle, c.to_string(), char_pos, line));
                },
                '>' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::RAngle, c.to_string(), char_pos, line));
                },
                '{' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::LBrace, c.to_string(), char_pos, line));
                },
                '}' => {
                    Self::check_word_number_end(&mut tokens, &mut current_word, &mut current_number, line, &mut word_start_pos, &mut number_start_pos);
                    tokens.push(Token::new(TokenType::RBrace, c.to_string(), char_pos, line));
                },
                _ => {
                    if c.is_numeric() {
                        if  current_word.len() != 0 {
                            current_word.push(c);
                        } else {
                            current_number.push(c);
                            if number_start_pos == 0 {
                                number_start_pos = char_pos;
                            }
                        }

                        continue;
                    }

                    if c.is_alphabetic() {
                        current_word.push(c);
                        if word_start_pos == 0 {
                            word_start_pos = char_pos;
                        }
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
                        char_pos = 0;
                    }
                }
            }

            char_pos += 1;
        }

        tokens
    }

    fn check_word_number_end(vec: &mut Vec<Token>, word: &mut String, number: &mut String, line: i32, word_pos: &mut i32, number_pos: &mut i32) {
        if word.len() > 0 {
            vec.push(Token::new(TokenType::Identifier, word.clone(), *word_pos, line));
            word.clear();
            *word_pos = 0;
        }
        if number.len() > 0 {
            vec.push(Token::new(TokenType::Number, number.clone(), *number_pos, line));
            number.clear();
            *number_pos = 0;
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