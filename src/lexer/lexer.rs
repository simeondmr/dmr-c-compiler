use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use crate::errors::errors::CompilerErrors;

pub static LEXER_SINGLETON: OnceLock<Mutex<Lexer>> = OnceLock::new();

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum Token {
    NumberU32(u32),
    Literal(String),
    SingleElem(char),
    RoundBracketOpen,
    RoundBracketClose,
    CurlyBracketOpen,
    CurlyBracketClose,
    Semicolon,
    //Start unary and binary operators
    BitwiseComplement,
    Negation,
    Add,
    Multiply,
    Divide,
    Reminder,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Not,
    Assignment,
    AssignmentAdd,
    AssignmentSub,
    AssignmentMultiply,
    AssignmentDivide,
    AssignmentReminder,
    AssignmentBitwiseOr,
    AssignmentBitwiseAnd,
    AssignmentBitwiseXor,
    AssignmentBitwiseLeftShift,
    AssignmentBitwiseRightShift,
    Increment,
    Decrement,
    Comma,
    QuestionMark,
    Colon,
    //End unary and binary operators
    Eof,
    Init,
    //Start keyword
    Int,
    Void,
    If,
    Else,
    Goto,
    Return,
    //end keyword
    Error
}

impl Token {
    pub fn extract_literal_val(&self) -> Option<String> {
        if let Token::Literal( val) = self {
            return Some(val.clone())
        }

        None
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Token::NumberU32(_), Token::NumberU32( _)) => true,
            (Token::Literal(_), Token::Literal(_)) => true,
            (Token::RoundBracketOpen, Token::RoundBracketOpen) => true,
            (Token::RoundBracketClose, Token::RoundBracketClose) => true,
            (Token::CurlyBracketOpen, Token::CurlyBracketOpen) => true,
            (Token::CurlyBracketClose, Token::CurlyBracketClose) => true,
            (Token::Semicolon, Token::Semicolon) => true,
            (Token::BitwiseComplement, Token::BitwiseComplement) => true,
            (Token::Negation, Token::Negation) => true,
            (Token::SingleElem(element0), Token::SingleElem(element1)) => if element0 == element1{ true } else { false },
            (Token::Assignment, Token::Assignment) => true,
            (Token::AssignmentAdd, Token::AssignmentAdd) => true,
            (Token::AssignmentSub, Token::AssignmentSub) => true,
            (Token::AssignmentMultiply, Token::AssignmentMultiply) => true,
            (Token::AssignmentDivide, Token::AssignmentDivide) => true,
            (Token::AssignmentReminder, Token::AssignmentReminder) => true,
            (Token::AssignmentBitwiseOr, Token::AssignmentBitwiseOr) => true,
            (Token::AssignmentBitwiseAnd, Token::AssignmentBitwiseAnd) => true,
            (Token::AssignmentBitwiseXor, Token::AssignmentBitwiseXor) => true,
            (Token::AssignmentBitwiseLeftShift, Token::AssignmentBitwiseLeftShift) => true,
            (Token::AssignmentBitwiseRightShift, Token::AssignmentBitwiseRightShift) => true,
            (Token::Increment, Token::Increment) => true,
            (Token::Decrement, Token::Decrement) => true,
            (Token::Comma, Token::Comma) => true,
            (Token::QuestionMark, Token::QuestionMark) => true,
            (Token::Colon, Token::Colon) => true,
            (Token::Int, Token::Int) => true,
            (Token::Void, Token::Void) => true,
            (Token::Return, Token::Return) => true,
            (Token::Eof, Token::Eof) => true,
            _ => false
        }
    }
}

pub struct Lexer {
    file: File,
    current_line: u32,
    current_token: Token,
    keywords: HashMap<String, Token>,
    lookahead_tokens: VecDeque<Token>
}

impl Lexer {
    pub fn new(file_name: &Path) -> Lexer {
        Lexer {
            file: File::open(file_name).expect("errore apertura file"),
            current_line: 1,
            current_token: Token::Init,
            keywords: HashMap::from([
                ("int".to_string(), Token::Int),
                ("void".to_string(), Token::Void),
                ("if".to_string(), Token::If),
                ("else".to_string(), Token::Else),
                ("goto".to_string(), Token::Goto),
                ("return".to_string(), Token::Return),
            ]),
            lookahead_tokens: VecDeque::new()
        }
    }
    
    pub fn feed_lookeheads_tokens(&mut self, n: u8) -> Result<(), CompilerErrors> {
        for _ in 0..n {
            let token = self.feed()?;
            self.lookahead_tokens.push_back(token);
        }
        Ok(())
    }
    
    pub fn peek_lookaheader_token(&mut self, n: usize) -> Option<&Token> {
        self.lookahead_tokens.get(n)
    }
    
    pub fn remove_lookahead(&mut self) -> Result<Option<Token>, CompilerErrors> {
        self.lookahead_tokens.pop_front();
        self.next_token()
    }
    
    pub fn next_token(&mut self) -> Result<Option<Token>, CompilerErrors> {
        let token = if !self.lookahead_tokens.is_empty() {
            self.lookahead_tokens.pop_front()
        } else {
            Some(self.feed()?)
        };
        self.current_token = token.clone().ok_or(CompilerErrors::LexicalError)?;
        Ok(token)
    }

    fn feed(&mut self) -> Result<Token, CompilerErrors> {
        let mut buf = [0];
        while self.file.read_exact(&mut buf).is_ok() {
            let c = buf[0] as char;

            if c == '\n' {
                self.current_line += 1;
                continue;
            }

            if c.is_whitespace() {
                continue;
            }

            if c.is_ascii_digit() {
                if (buf[0] as char).is_ascii_digit() {
                    let mut number = c.to_string();
                    let mut last_char = None;

                    while self.file.read_exact(&mut buf).is_ok() {
                        let next_c = buf[0] as char;
                        if next_c.is_ascii_digit() {
                            number.push(next_c);
                        } else {
                            self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                            last_char = Some(next_c);
                            break;
                        }
                    }

                    //Number inbound check
                    if let Some(character) = last_char {
                        if character.is_ascii_alphabetic() {
                            return Err(CompilerErrors::LexicalError)
                        }
                    }
                    return Ok(Token::NumberU32(number.parse().expect("Expected number")))
                } else {
                    self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                }
            }
            if c.is_ascii_alphabetic() || c == '_' {
                let mut ident = c.to_string();

                let mut is_eof = false;

                loop {
                    let read_value = self.file.read_exact(&mut buf);
                    if !(buf[0] as char).is_ascii_alphanumeric() && (buf[0] as char) != '_' {
                        break;
                    }

                    if let Ok(_) = read_value {
                        ident.push(buf[0] as char);
                    } else {
                        is_eof = true;
                        break;
                    }
                }

                if !is_eof {
                    self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                }
                
                return Ok(self.keywords.get(&ident).cloned().unwrap_or_else(|| Token::Literal(ident)));
            }
            if c == '<' {
                //we need to read another character in order to understand if the operator is < or <<
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '<' {
                        let read_value = self.file.read_exact(&mut buf);
                        if let Ok(_) = read_value {
                            if buf[0] as char == '=' {
                                return Ok(Token::AssignmentBitwiseLeftShift)
                            }
                        }
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        //bitwise left shift
                        return Ok(Token::BitwiseLeftShift)
                    } else if buf[0] as char == '=' {
                        return Ok(Token::LessThanOrEqual)
                    } else {
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        return Ok(Token::LessThan)
                    }
                } else {
                    //EOF
                    break;
                }
            }
            if c == '>' {
                //we need to read another character in order to understand if the operator is < or <<
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '>' {
                        let read_value = self.file.read_exact(&mut buf);
                        if let Ok(_) = read_value {
                            if buf[0] as char == '=' {
                                return Ok(Token::AssignmentBitwiseRightShift)
                            }
                        }
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        //bitwise left shift
                        return Ok(Token::BitwiseRightShift)
                    } else if buf[0] as char == '=' {
                        return Ok(Token::GreaterThanOrEqual)
                    } else {
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        return Ok(Token::GreaterThan)
                    }
                } else {
                    //EOF
                    break;
                }
            }
            if c == '=' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '=' {
                        // Equal operator
                        self.current_token = Token::Equal;
                        return Ok(self.current_token.clone())
                    } else {
                        // Assignation
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        return Ok(Token::Assignment)
                    }
                }
            }
            if c == '!' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '=' {
                        //not equal
                        return Ok(Token::NotEqual)
                    } else {
                        //logical Not
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        return Ok(Token::Not)
                    }
                }
            }
            if c == '{' {
                return Ok(Token::CurlyBracketOpen)
            }
            if c == '}' {
                return Ok(Token::CurlyBracketClose)
            }
            if c == '(' {
                return Ok(Token::RoundBracketOpen)
            }
            if c == ')' {
                return Ok(Token::RoundBracketClose)
            }
            if c == ';' {
                return Ok(Token::Semicolon)
            }
            if c == '~' {
                return Ok(Token::BitwiseComplement)
            }
            if c == '+' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '+' {
                        return Ok(Token::Increment)
                    }   
                    if buf[0] as char == '=' {
                        return Ok(Token::AssignmentAdd)
                    }
                }
                self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                return Ok(Token::Add)
            }
            if c == '-' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '-' {
                        return Ok(Token::Decrement)
                    }
                    if buf[0] as char == '=' {
                        return Ok(Token::AssignmentSub)
                    }
                }
                self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                return Ok(Token::Negation)
            }
            if c == '*' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '=' {
                        return Ok(Token::AssignmentMultiply)
                    }
                }
                self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                return Ok(Token::Multiply)
            }
            if c == '/' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '=' {
                        return Ok(Token::AssignmentDivide)
                    }
                }
                self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                return Ok(Token::Divide)
            }
            if c == '%' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '=' {
                        return Ok(Token::AssignmentReminder)
                    }
                }
                self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                return Ok(Token::Reminder)
            }
            if c == '&' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '&' {
                        // Logical and
                        return Ok(Token::And)
                    } else if buf[0] as char == '=' {
                        return Ok(Token::AssignmentBitwiseAnd)
                    } else {
                        // Bitwise and
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        return Ok(Token::BitwiseAnd)
                    }
                }
            }
            if c == '|' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '|' {
                        // Logical or
                        return Ok(Token::Or)
                    } else if buf[0] as char == '=' {
                        return Ok(Token::AssignmentBitwiseOr)
                    } else {
                        // Bitwise or
                        self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                        return Ok(Token::BitwiseOr)
                    }
                }
            }
            if c == '^' {
                let read_value = self.file.read_exact(&mut buf);
                if let Ok(_) = read_value {
                    if buf[0] as char == '=' {
                        return Ok(Token::AssignmentBitwiseXor)
                    }
                }
                self.file.seek(SeekFrom::Current(-1)).expect("Failed to seek back");
                return Ok(Token::BitwiseXor)
            }
            if c == ',' {
                return Ok(Token::Comma)
            }
            if c == '?' {
                return Ok(Token::QuestionMark)
            }
            if c == ':' {
                return Ok(Token::Colon)
            }
            eprintln!("Error at line {}: unknow symbol: {}", self.current_line, c);
            return Err(CompilerErrors::LexicalError)
        }
        Ok(Token::Eof)
    }

    pub fn current_line(&self) -> u32 {
        self.current_line
    }

    pub fn current_token(&self) -> Token {
        self.current_token.clone()
    }
}