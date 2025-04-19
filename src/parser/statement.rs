use crate::ast::lang_ast::statement_node::StatementNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::expr::Expr;
use crate::parser::program::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct Statement {
    exp: Expr
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            exp: Expr::new()
        }
    }
}

impl GrammarProductionParsing<StatementNode> for Statement {
    fn parse(&self) -> Result<StatementNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::Return, &mut lexer)?;
        drop(lexer);
        let constant_ast  = self.exp.parse(0)?;
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::Semicolon, &mut lexer)?;
        Ok(StatementNode::ReturnStmt(constant_ast))
    }
}