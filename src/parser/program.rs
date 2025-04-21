use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, LEXER_SINGLETON, Token};
use crate::parser::function::Function;

pub trait GrammarProductionParsing<T> {
    fn parse(&self) -> Result<T, CompilerErrors>;

    fn match_token(expected_token: &Token, lexer: &mut Lexer) -> Result<(), CompilerErrors> {
        if *expected_token == lexer.current_token() {
            lexer.next_token()?;
            return Ok(())
        }

        eprintln!("Error at line {}: expected {:?}, but found {:?}", lexer.current_line(), expected_token, lexer.current_token());
        Err(CompilerErrors::SyntaxError)
    }

    ///Use this method if you wanna lock the lexer
    fn lexer_lock() -> MutexGuard<'static, Lexer> {
        LEXER_SINGLETON.get().unwrap().lock().unwrap()
    }

    ///Use this methods if you don't want to lock the lexer
    fn lexer() -> &'static Mutex<Lexer> {
        LEXER_SINGLETON.get().unwrap()
    }
}

#[allow(dead_code)]
pub trait PrecedenceClimbingParsing<T> {
    fn parse(&self, min_prec: u8) -> Result<T, CompilerErrors>;

    fn match_token(expected_token: &Token, lexer: &mut Lexer) -> Result<(), CompilerErrors> {
        if *expected_token == lexer.current_token() {
            lexer.next_token()?;
            return Ok(())
        }

        eprintln!("Error at line {}: expected {:?}, but found {:?}", lexer.current_line(), expected_token, lexer.current_token());
        Err(CompilerErrors::SyntaxError)
    }

    ///Use this method if you wanna lock the lexer
    fn lexer_lock() -> MutexGuard<'static, Lexer> {
        LEXER_SINGLETON.get().unwrap().lock().unwrap()
    }

    ///Use this methods if you don't want to lock the lexer
    fn lexer() -> &'static Mutex<Lexer> {
        LEXER_SINGLETON.get().unwrap()
    }

    fn is_operator(operator: &Token) -> bool {
        match operator {
            Token::BitwiseComplement => true,
            Token::Negation => true,
            Token::Add => true,
            Token::Multiply => true,
            Token::Divide => true,
            Token::Reminder => true,
            Token::BitwiseAnd => true,
            Token::BitwiseOr => true,
            Token::BitwiseXor => true,
            Token::BitwiseLeftShift => true,
            Token::BitwiseRightShift => true,
            _ => false
        }
    }

    fn operator_precedence(operator: &Token) -> Result<u8, CompilerErrors> {
        match operator {
            Token::BitwiseComplement => Ok(0),
            Token::Negation => Ok(50),
            Token::Add => Ok(50),
            Token::Multiply => Ok(60),
            Token::Divide => Ok(60),
            Token::Reminder => Ok(60),
            Token::BitwiseAnd => Ok(0),
            Token::BitwiseOr => Ok(0),
            Token::BitwiseXor => Ok(0),
            Token::BitwiseLeftShift => Ok(40),
            Token::BitwiseRightShift => Ok(40),
            _ => Err(CompilerErrors::OperatorPrecedenceError)
        }
    }
}

pub struct Program<'a> {
    input_path: &'a Path,
    function: Function
}

impl <'a> Program <'a> {
    pub fn new(input_path: &Path) -> Program {
        Program {
            function: Function::new(),
            input_path
        }
    }
}

impl <'a> GrammarProductionParsing<ProgramNode> for Program<'a> {
    fn parse(&self) -> Result<ProgramNode, CompilerErrors> {
        LEXER_SINGLETON.get_or_init(|| Mutex::new(Lexer::new(self.input_path)));
        Self::lexer().lock().unwrap().next_token()?;
        let function_node = self.function.parse()?;
        Self::match_token(&Token::Eof, &mut Self::lexer_lock())?;
        Ok(ProgramNode::ProgramDef(function_node))
    }
}