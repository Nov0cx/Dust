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
    Number,
    EOF
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

#[derive(Debug, Clone)]
pub struct Lexer {
    source: String,
    pos: usize,
    char_pos: i32,
    line: i32,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        Lexer {
            source,
            pos: 0,
            char_pos: 1,
            line: 1
        }
    }

    fn peek_char(&self) -> char {
        self.source.chars().nth(self.pos + 1).unwrap()
    }

    pub fn try_token(&mut self) -> Option<Token> {
        let c = if let Some(ch) = self.source.chars().nth(self.pos) {
            ch
        } else {
            return Some(Token::new(TokenType::EOF, "".to_string(), self.char_pos, self.line));
        };
        self.pos += 1;

        match c {
            '(' => {
                return Some(Token::new(TokenType::LParen, c.to_string(), self.char_pos, self.line));
            }
            ')' => {
                return Some(Token::new(TokenType::RParen, c.to_string(), self.char_pos, self.line));
            }
            '{' => {
                return Some(Token::new(TokenType::LBrace, c.to_string(), self.char_pos, self.line));
            }
            '}' => {
                return Some(Token::new(TokenType::RBrace, c.to_string(), self.char_pos, self.line));
            }
            ',' => {
                return Some(Token::new(TokenType::Comma, c.to_string(), self.char_pos, self.line));
            }
            '<' => {
                return Some(Token::new(TokenType::LAngle, c.to_string(), self.char_pos, self.line));
            }
            '>' => {
                return Some(Token::new(TokenType::RAngle, c.to_string(), self.char_pos, self.line));
            }
            ':' => {
                return Some(Token::new(TokenType::Colon, c.to_string(), self.char_pos, self.line));
            }
            ';' => {
                return Some(Token::new(TokenType::Semicolon, c.to_string(), self.char_pos, self.line));
            }
            '0'..='9' => {
                let mut value = c.to_string();

                let current = self.source.chars().nth(self.pos).unwrap();
                if current.is_digit(10) {
                    value.push(current);
                }


                self.pos += 1;
                let mut next = self.source.chars().nth(self.pos).unwrap();

                while next.is_digit(10) || (next == '_' && self.peek_char().is_digit(10)) {
                    value.push(next);
                    self.pos += 1;
                    next = self.source.chars().nth(self.pos).unwrap();
                }

                return Some(Token {
                    token_type: TokenType::Number,
                    value,
                    char_pos: self.char_pos,
                    line: self.line
                });

            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut value = c.to_string();

                let current = self.source.chars().nth(self.pos).unwrap();
                if current.is_alphanumeric() || current == '_' {
                    value.push(current);
                }

                self.pos += 1;

                let mut next = self.source.chars().nth(self.pos).unwrap();
                while next.is_alphanumeric() || next == '_' {
                    value.push(next);
                    self.pos += 1;
                    next = self.source.chars().nth(self.pos).unwrap();
                }

                return Some(Token {
                    token_type: TokenType::Identifier,
                    value,
                    char_pos: self.char_pos,
                    line: self.line
                });
            }
            '\0' => {
                return Some(Token {
                    token_type: TokenType::EOF,
                    value: "".to_string(),
                    char_pos: self.char_pos,
                    line: self.line
                });
            }
            _ => {
                let current = self.source.chars().nth(self.pos).unwrap();
                if current == '\n' {
                    self.line += 1;
                    self.char_pos = 1;
                }

                None
            }
        }
    }

    pub fn peek(&mut self) -> Token {
        // saving the current position
        let current_pos = self.pos;
        let current_char_pos = self.char_pos;
        let current_line = self.line;

        // retrieving the token
        let token = self.next();

        // restoring the position
        self.pos = current_pos;
        self.char_pos = current_char_pos;
        self.line = current_line;

        token
    }

    pub fn next(&mut self) -> Token {

        let mut token = self.try_token();

        while token.is_none() {
            token = self.try_token();
        }

        println!("{:?}", token);
        token.unwrap()
    }

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