
use crate::{Lexer, Token, TokenType};
use crate::pair::Pair;

#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub subtype: Option<Box<Type>>
}

#[derive(Debug)]
pub enum AST {
    File { child: Vec<AST> },
    Return { value: Box<AST> },
    Value { value: String },
    FunctionCall { name: String, args: Vec<AST> },
    FunctionDefinition { name: String, args: Vec<Pair<String, Type>>, body: Vec<AST>, return_type: String },
    None,
}



#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
    current_token: Token,
}

impl Parser {

    pub fn new(lexer: Lexer) -> Parser {
        Self {
            lexer,
            current_token: Token::new(TokenType::EOF, "".to_string(), -1, -1),
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut file = AST::File {
            child: Vec::new()
        };

        let mut current_node = AST::None;
        
        let mut token = self.lexer.next();
        while token.token_type != TokenType::EOF {
            match token.token_type {
                TokenType::Identifier => {
                    let colon = self.lexer.next();
                    if colon.token_type != TokenType::Colon {
                        self.error_with_string(colon.line, colon.char_pos, format!("Expected ':' after identifier '{}' but got '{}'", token.value, colon.value));
                        break;
                    }

                    let typename = self.lexer.next();

                    if typename.value == "func" {
                        let mut args: Vec<Pair<String, Type>> = Vec::new();
                        let mut body = Vec::new();

                        let mut next = self.lexer.next();

                        let mut return_type = "void".to_string();

                        // parse arguments
                        match next.token_type {
                            TokenType::LParen => {
                                next = self.lexer.next();
                                if next.token_type != TokenType::RParen {
                                    if next.token_type == TokenType::Identifier {

                                        let mut arg_pair = Pair {
                                            0: next.value.clone(),
                                            1: Type {
                                                name: "".to_string(),
                                                subtype: None
                                            }
                                        };

                                        let mut index = 0;


                                        println!("{:?}", next);

                                        while next.token_type != TokenType::RParen {

                                            if next.token_type == TokenType::Comma {
                                                index = 0;
                                                args.push(arg_pair.clone());
                                                next = self.lexer.next();
                                            }

                                            if index == 1 {
                                                if next.token_type == TokenType::Colon {
                                                    next = self.lexer.next();
                                                } else {
                                                    self.error_with_string(next.line, next.char_pos,
                                                                           format!("Expected ':' after identifier '{}' but got '{}'", arg_pair.0, next.value));
                                                }


                                                let peek = self.lexer.peek();

                                                if next.token_type == TokenType::Identifier {
                                                    // not acounting nested subtypes
                                                    if peek.token_type == TokenType::LAngle {
                                                        arg_pair.1.name = next.value.clone();

                                                        self.lexer.next();
                                                        next = self.lexer.next();
                                                        self.lexer.next();

                                                        let subtype = Some(Box::new(Type {
                                                            name: next.value.clone(),
                                                            subtype: None
                                                        }));
                                                        arg_pair.1.subtype = subtype;
                                                    } else {
                                                        arg_pair.1 = Type {
                                                            name: next.value.clone(),
                                                            subtype: None
                                                        };
                                                    }
                                                } else {
                                                    self.error_with_string(next.line, next.char_pos,
                                                                           format!("Expected identifier after ',' in argument list got '{}'", next.value));
                                                    break;
                                                }
                                            } else if index == 0 {
                                                arg_pair.0 = next.value.clone();
                                            } else {
                                                self.error_with_string(next.line, next.char_pos, format!("Expected ',' after argument '{:?}'", arg_pair.1));
                                                break;
                                            }

                                            index += 1;
                                            next = self.lexer.next();
                                        }

                                        if index != 2 {
                                            self.error(next.line, next.char_pos, "The argument list has a wrong format");
                                            break;
                                        }

                                        args.push(arg_pair.clone());

                                        // should be unreachable but just in case
                                        if next.token_type != TokenType::RParen {
                                            self.error_with_string(next.line, next.char_pos, format!("Expected ')' after params but got '{}'", next.value));
                                            break;
                                        }
                                    } else {
                                        self.error_with_string(next.line, next.char_pos, format!("Expected identifier after '(' but got '{}'", next.value));
                                        break;
                                    }
                                }
                            }
                            _ => {
                                // TODO functions without arguments
                                self.error(next.line, next.char_pos, "Expected '(' after 'func'");
                            }
                        }

                        // get return type
                        next = self.lexer.next();
                        if next.token_type == TokenType::Colon {
                            next = self.lexer.next();
                            return_type = next.value;
                        }

                        next = self.lexer.next();
                        if next.token_type != TokenType::LBrace {
                            self.error_with_string(next.line, next.char_pos, format!("Expected '{}' after return type", return_type));
                            break;
                        }

                        // parse body
                        next = self.lexer.next();
                        while next.token_type != TokenType::RBrace {
                            match next.token_type {
                                TokenType::Identifier => {
                                    if next.value == "return" {
                                        let return_value = self.lexer.next();
                                        if return_value.token_type != TokenType::Semicolon {
                                            body.push(AST::Return {
                                                value: Box::new(AST::Value { value: return_value.value })
                                            });
                                            next = self.lexer.next();
                                            break;
                                        } else {
                                            if return_type != "void" {
                                                self.error(return_value.line, return_value.char_pos, "Expected return value after 'return'");
                                                break;
                                            }
                                            body.push(AST::Return {
                                                value: Box::new(AST::Value {
                                                    value: "".to_string()
                                                })
                                            });
                                            next = self.lexer.next();
                                            break;
                                        }
                                    }
                                    self.error(next.line, next.char_pos, "Not implemented");
                                }
                                _ => {}
                            }

                            next = self.lexer.next();
                        }

                        if next.token_type != TokenType::RBrace {
                            self.error(next.line, next.char_pos, "Not closing function.");
                            break;
                        }

                        current_node = AST::FunctionDefinition {
                            name: token.value.clone(),
                            args,
                            body,
                            return_type,
                        };

                        match file {
                            AST::File { mut child } => {
                                child.push(current_node);
                                file = AST::File {
                                    child
                                };
                            }
                            _ => {
                                panic!("Unreachable");
                            }
                        }
                    }
                }
                _ => {
                    self.error_with_string(token.line, token.char_pos, format!("Unexpected token: {:?}", token.token_type));
                }
            }
            token = self.lexer.next();
        }

        file
    }

    fn error(&self, line: i32, char_pos: i32, msg: &str) {
        eprintln!("[Parser] Error at {line}:{char_pos}: {}", msg);
        panic!();
    }

    fn error_with_string(&self, line: i32, char_pos: i32, msg: String) {
        eprintln!("[Parser] Error at {line}:{char_pos}: {}", msg);
        panic!();
    }
}