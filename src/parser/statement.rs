use crate::ast::lang_ast::statement_node::StatementNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::exp::Exp;
use crate::parser::program::{GrammarProductionParsing};

pub struct Statement {
    exp: Exp
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            exp: Exp::new()
        }
    }
}


impl GrammarProductionParsing<StatementNode> for Statement {
    fn parse(&self) -> Result<StatementNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::Return, &mut lexer)?;
        drop(lexer);
        let constant_ast  = self.exp.parse()?;
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::Semicolon, &mut lexer)?;
        Ok(StatementNode::ReturnStmt(constant_ast))
    }
}