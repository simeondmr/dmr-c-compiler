use crate::ast::lang_ast::exp_node::ExpNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};

pub struct Exp;

impl Exp {
    pub fn new() -> Exp {
        Exp {

        }
    }
}

impl GrammarProductionParsing<ExpNode> for Exp {
    fn parse(&self) -> Result<ExpNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();
        Self::match_token(&Token::NumberU32(0), &mut lexer)?;
        return Ok(ExpNode::Constant(current_token.extract_i32_val().unwrap()));
    }
}