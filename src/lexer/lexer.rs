use std::collections::HashMap;
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
    //Start unary operators
    BitwiseComplement,
    Negation,
    //End unary operators
    Eof,
    Init,
    //Start keyword
    Int,
    Void,
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
    keywords: HashMap<String, Token>
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
                ("return".to_string(), Token::Return),
            ])
        }
    }

    pub fn next_token(&mut self) -> Result<Token, CompilerErrors> {
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

                    self.current_token = Token::NumberU32(number.parse().expect("Expected number"));
                    return Ok(self.current_token.clone())
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


                self.current_token = self.keywords.get(&ident).cloned().unwrap_or_else(|| Token::Literal(ident));
                return Ok(self.current_token.clone());
            }

            if c == '{' {
                self.current_token = Token::CurlyBracketOpen;
                return Ok(self.current_token.clone())
            }

            if c == '}' {
                self.current_token = Token::CurlyBracketClose;
                return Ok(self.current_token.clone())
            }

            if c == '(' {
                self.current_token = Token::RoundBracketOpen;
                return Ok(self.current_token.clone())
            }

            if c == ')' {
                self.current_token = Token::RoundBracketClose;
                return Ok(self.current_token.clone())
            }

            if c == ';' {
                self.current_token = Token::Semicolon;
                return Ok(self.current_token.clone())
            }

            if c == '~' {
                self.current_token = Token::BitwiseComplement;
                return Ok(self.current_token.clone())
            }

            if c == '-' {
                self.current_token = Token::Negation;
                return Ok(self.current_token.clone())
            }

            eprintln!("Error at line {}: unknow symbol: {}", self.current_line, c);
            return Err(CompilerErrors::LexicalError)
        }

        self.current_token = Token::Eof;
        Ok(Token::Eof)
    }

    pub fn current_line(&self) -> u32 {
        self.current_line
    }

    pub fn current_token(&self) -> Token {
        self.current_token.clone()
    }
}