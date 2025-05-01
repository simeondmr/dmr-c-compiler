use crate::ast::lang_ast::expr_node::ExprNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::expr::Expr;
use crate::parser::program::{GrammarProductionParsing, PrecedenceClimbingParsing};
use crate::parser::unop::Unop;

pub struct Factor {
    unop: Unop,
}

impl Factor {
    pub fn new() -> Factor {
        Factor {
            unop: Unop::new(),
        }
    }
}

impl GrammarProductionParsing<ExprNode> for Factor {
    fn parse(&self) -> Result<ExprNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();
        match current_token {
            Token::NumberU32(value) => {
                lexer.next_token()?;
                Ok(ExprNode::Constant(value as i32))
            },
            Token::BitwiseComplement | Token::Negation | Token::Not => {
                drop(lexer);
                let expr = Expr::new();
                Ok(ExprNode::Unary { unary_operator: self.unop.parse()?, expr: Box::new(expr.parse(0)?) })
            },
            Token::RoundBracketOpen => {
                lexer.next_token()?;
                drop(lexer);
                let expr = Expr::new();
                let expr_node = expr.parse(0);
                Self::match_token(&Token::RoundBracketClose, &mut Self::lexer_lock())?;
                expr_node
            },
            _ => {
                eprintln!("Syntax error at line {:?}: unexpected {:?} token", lexer.current_line(), current_token);
                Err(CompilerErrors::SyntaxError)
            }
        }
    }
}