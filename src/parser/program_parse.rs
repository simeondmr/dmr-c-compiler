use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use crate::ast::lang_ast::expr_node::AssignmentOperatorType;
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, LEXER_SINGLETON, Token};
use crate::parser::function_parse::FunctionParse;

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

    fn is_pre_post_operator(operator: &Token) -> bool {
        match operator {
            Token::Increment => true,
            Token::Decrement => true,
            _ => false
        }
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
            Token::Equal => true,
            Token::NotEqual => true,
            Token::LessThan => true,
            Token::LessThanOrEqual => true,
            Token::GreaterThan => true,
            Token::GreaterThanOrEqual => true,
            Token::And => true,
            Token::Or => true,
            Token::Not => true,
            Token::Assignment => true,
            Token::AssignmentAdd => true,
            Token::AssignmentSub => true,
            Token::AssignmentMultiply => true,
            Token::AssignmentDivide => true,
            Token::AssignmentReminder => true,
            Token::AssignmentBitwiseOr => true,
            Token::AssignmentBitwiseAnd => true,
            Token::AssignmentBitwiseXor => true,
            Token::AssignmentBitwiseLeftShift => true,
            Token::AssignmentBitwiseRightShift => true,
            _ => false
        }
    }
    
    fn is_assignment_operator(operator: &Token) -> Option<AssignmentOperatorType> {
        match operator {
            Token::Assignment => Some(AssignmentOperatorType::AssignmentDefault),
            Token::AssignmentAdd => Some(AssignmentOperatorType::AssignmentAdd),
            Token::AssignmentSub => Some(AssignmentOperatorType::AssignmentSub),
            Token::AssignmentMultiply => Some(AssignmentOperatorType::AssignmentMultiply),
            Token::AssignmentDivide => Some(AssignmentOperatorType::AssignmentDivide),
            Token::AssignmentReminder => Some(AssignmentOperatorType::AssignmentReminder),
            Token::AssignmentBitwiseOr => Some(AssignmentOperatorType::AssignmentBitwiseOr),
            Token::AssignmentBitwiseAnd => Some(AssignmentOperatorType::AssignmentBitwiseAnd),
            Token::AssignmentBitwiseXor => Some(AssignmentOperatorType::AssignmentBitwiseXor),
            Token::AssignmentBitwiseLeftShift => Some(AssignmentOperatorType::AssignmentBitwiseLeftShift),
            Token::AssignmentBitwiseRightShift => Some(AssignmentOperatorType::AssignmentBitwiseRightShift),
            _ => None
        }
    }

    fn operator_precedence(operator: &Token) -> Result<u8, CompilerErrors> {
        match operator {
            Token::BitwiseComplement => Ok(60),
            Token::Negation => Ok(50),
            Token::Add => Ok(50),
            Token::Multiply => Ok(60),
            Token::Divide => Ok(60),
            Token::Reminder => Ok(60),
            Token::BitwiseAnd => Ok(45),
            Token::BitwiseOr => Ok(45),
            Token::BitwiseXor => Ok(45),
            Token::BitwiseLeftShift => Ok(55),
            Token::BitwiseRightShift => Ok(55),
            Token::Equal => Ok(30),
            Token::NotEqual => Ok(30),
            Token::LessThan => Ok(35),
            Token::LessThanOrEqual => Ok(35),
            Token::GreaterThan => Ok(35),
            Token::GreaterThanOrEqual => Ok(35),
            Token::And => Ok(10),
            Token::Or => Ok(5),
            Token::Assignment => Ok(1),
            Token::AssignmentAdd => Ok(1),
            Token::AssignmentSub => Ok(1),
            Token::AssignmentMultiply => Ok(1),
            Token::AssignmentDivide => Ok(1),
            Token::AssignmentReminder => Ok(1),
            Token::AssignmentBitwiseOr => Ok(1),
            Token::AssignmentBitwiseAnd => Ok(1),
            Token::AssignmentBitwiseXor => Ok(1),
            Token::AssignmentBitwiseLeftShift => Ok(1),
            Token::AssignmentBitwiseRightShift => Ok(1),
            _ => Err(CompilerErrors::OperatorPrecedenceError)
        }
    }
}

pub struct ProgramParse<'a> {
    input_path: &'a Path,
    function_parse: FunctionParse
}

impl <'a> ProgramParse <'a> {
    pub fn new(input_path: &Path) -> ProgramParse {
        ProgramParse {
            function_parse: FunctionParse::new(),
            input_path
        }
    }
}

impl <'a> GrammarProductionParsing<ProgramNode> for ProgramParse<'a> {
    fn parse(&self) -> Result<ProgramNode, CompilerErrors> {
        LEXER_SINGLETON.get_or_init(|| Mutex::new(Lexer::new(self.input_path)));
        Self::lexer().lock().unwrap().next_token()?;
        let function_node = self.function_parse.parse()?;
        Self::match_token(&Token::Eof, &mut Self::lexer_lock())?;
        Ok(ProgramNode::ProgramDef(function_node))
    }
}