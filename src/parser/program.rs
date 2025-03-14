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

pub struct Program {
    function: Function
}

impl Program {
    pub fn new() -> Program {
        Program {
            function: Function::new(),
        }
    }
}

impl GrammarProductionParsing<ProgramNode> for Program {
    fn parse(&self) -> Result<ProgramNode, CompilerErrors> {
        LEXER_SINGLETON.get_or_init(|| Mutex::new(Lexer::new("C:\\Users\\stornabene\\IdeaProjects\\IdeaProjects\\Rust2024Projects\\dmr_c_compiler\\examples\\test.c".to_string())));
        Self::lexer().lock().unwrap().next_token()?;
        let function_node = self.function.parse()?;
        Self::match_token(&Token::Eof, &mut Self::lexer_lock())?;
        Ok(ProgramNode::ProgramDef(function_node))
    }
}