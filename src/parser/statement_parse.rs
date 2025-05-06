use crate::ast::lang_ast::statement_node::StatementNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::expr_parse::ExprParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct Statement {
    expr_parse: ExprParse
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            expr_parse: ExprParse::new()
        }
    }
}

impl GrammarProductionParsing<StatementNode> for Statement {
    fn parse(&self) -> Result<StatementNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();
        match current_token {
            Token::Return => {
                lexer.next_token()?;
                drop(lexer);
                let expr_ast  = self.expr_parse.parse(0)?;
                let mut lexer = Self::lexer_lock();
                Self::match_token(&Token::Semicolon, &mut lexer)?;
                Ok(StatementNode::ReturnStmt(expr_ast))
            },
            Token::Semicolon => {
                lexer.next_token()?;
                Ok(StatementNode::EmptyStmt)
            },
            _ => {
                // Note: if is not a return stmt and neither an Empty stmt so must be an expression
                drop(lexer);
                let expr_ast = self.expr_parse.parse(0)?;
                let mut lexer = Self::lexer_lock();
                Self::match_token(&Token::Semicolon, &mut lexer)?;
                Ok(StatementNode::Expr(expr_ast))
            }
        }
    }
}