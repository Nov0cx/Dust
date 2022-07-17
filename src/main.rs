mod lexer;

use std::*;
use lexer::{Token, Lexer, TokenType};

#[derive(Debug)]
enum AST {
    File { child: Vec<AST> },
    Return { value: Box<AST> },
    Value { value: String },
    FunctionCall { name: String, args: Vec<AST> },
    FunctionDefinition { name: String, args: Vec<Pair<String, String>>, body: Vec<AST>, return_type: String },
    None,
}

#[derive(Debug, Clone)]
struct Pair<A, B>(A, B);

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn with_lexer(src: String) -> Parser {
        Parser {
            tokens: Lexer::tokenize(src),
            pos: 0,
        }
    }

    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> AST {
        let mut file = AST::File {
            child: Vec::new()
        };

        let mut current_node = AST::None;

        while self.pos < self.tokens.len() {
            let token = self.current_token().clone();

            match token.token_type {
                TokenType::Identifier => {
                    let colon = self.advance_with_token().unwrap().clone();
                    if colon.token_type != TokenType::Colon {
                        self.error_with_string(colon.line, colon.char_pos, format!("Expected ':' after identifier '{}'", token.value));
                        break;
                    }

                    let typename = self.advance_with_token().unwrap().clone();

                    if typename.value == "func" {
                        let mut args: Vec<Pair<String, String>> = Vec::new();
                        let mut body = Vec::new();

                        let mut next = self.advance_with_token().unwrap().clone();

                        let mut return_type = "void".to_string();

                        // parse arguments
                        match next.token_type {
                            TokenType::LParen => {
                                next = self.advance_with_token().unwrap().clone();
                                let mut arg: Pair<String, String> = Pair("".to_string(), "".to_string());
                                let mut parse_index = 0;
                                while next.token_type != TokenType::RParen {
                                    if next.token_type == TokenType::LAngle {
                                        arg.1 = format!("{}<{}>", arg.1, self.advance_with_token().unwrap().clone().value);
                                        self.pos += 1;
                                        args.push(arg.clone());
                                        parse_index = 0;
                                    }

                                    if parse_index == 0 {
                                        arg.0 = next.value;
                                    } else if parse_index == 1 {
                                        if next.token_type == TokenType::Colon {
                                            next = self.advance_with_token().unwrap().clone();
                                        } else {
                                            self.error_with_string(next.line, next.char_pos, format!("Expected ':' after identifier '{}'", arg.0));
                                            break;
                                        }
                                        arg.1 = next.value;
                                    } else {
                                        self.error_with_string(next.line, next.char_pos, format!("Expected identifier after '{}'", arg.0));
                                        self.error_with_string(next.line, next.char_pos, format!("Expected ')' after function definition"));
                                        break;
                                    }

                                    next = self.advance_with_token().unwrap().clone();
                                    if next.token_type == TokenType::Comma {
                                        args.push(arg.clone());
                                        next = self.advance_with_token().unwrap().clone();
                                        parse_index = 0;
                                    } else { parse_index += 1; }
                                }
                            }
                            _ => {
                                // TODO functions without arguments
                                self.error(next.line, next.char_pos, "Expected '(' after 'func'");
                            }
                        }

                        // get return type
                        next = self.advance_with_token().unwrap().clone();
                        if next.token_type == TokenType::Colon {
                            next = self.advance_with_token().unwrap().clone();
                            return_type = next.value;
                        }

                        next = self.advance_with_token().unwrap().clone();
                        if next.token_type != TokenType::LBrace {
                            self.error_with_string(next.line, next.char_pos, format!("Expected '{}' after return type", return_type));
                            break;
                        }

                        // parse body
                        next = self.advance_with_token().unwrap().clone();
                        while next.token_type != TokenType::RBrace {
                            match next.token_type {
                                TokenType::Identifier => {
                                    if next.value == "return" {
                                        let mut return_value = self.advance_with_token().unwrap().clone();
                                        if return_value.token_type != TokenType::Semicolon {
                                            body.push(AST::Return {
                                                value: Box::new(AST::Value { value: return_value.value })
                                            });
                                            self.pos += 1;
                                            next = self.advance_with_token().unwrap().clone();
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
                                            self.pos += 1;
                                            next = self.advance_with_token().unwrap().clone();
                                            break;
                                        }
                                    }
                                    self.error(next.line, next.char_pos, "Not implemented");
                                }
                                _ => {}
                            }

                            next = self.advance_with_token().unwrap().clone();
                        }

                        if next.token_type != TokenType::RBrace {
                            println!("{:?}", next);
                            self.error(next.line, next.char_pos, "Not closing function.");
                            break;
                        }

                        current_node = AST::FunctionDefinition {
                            name: token.value,
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

            self.advance();
        }

        file
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn advance_with_token(&mut self) -> Option<&Token> {
        self.pos += 1;
        if self.pos < self.tokens.len() {
            return Some(&self.tokens[self.pos]);
        }
        None
    }

    fn current_token(&mut self) -> &Token {
        &self.tokens[self.pos]
    }

    fn error(&self, line: i32, char_pos: i32, msg: &str) {
        eprintln!("[Parser] Error at {line}:{char_pos}: {}", msg);
    }

    fn error_with_string(&self, line: i32, char_pos: i32, msg: String) {
        eprintln!("[Parser] Error at {line}:{char_pos}: {}", msg);
    }
}

fn print_command_usage(program: String) {
    eprintln!("Usage: {} <file>", program);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() != 2 {
        print_command_usage(program);
        return;
    }

    let file = fs::read_to_string(&args[1]).unwrap();
    //println!("File: {}", file);
    let tokens = Lexer::tokenize(file.to_string());
    //println!("Tokens: {:?}", tokens);

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    println!("AST: {:?}", ast);
}
