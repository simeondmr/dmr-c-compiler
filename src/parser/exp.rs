use crate::ast::lang_ast::exp_node::ExpNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};
use crate::parser::unop::Unop;

pub struct Exp {
    unop: Unop
}

impl Exp {
    pub fn new() -> Exp {
        Exp {
            unop: Unop::new()
        }
    }
}

impl GrammarProductionParsing<ExpNode> for Exp {
    fn parse(&self) -> Result<ExpNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();

        match current_token {
            Token::NumberU32(value) => {
                lexer.next_token()?;
                Ok(ExpNode::Constant(value as i32))
            },
            Token::BitwiseComplement | Token::Negation => {
                drop(lexer);
                let unop_node = self.unop.parse()?;
                Ok(ExpNode::Unary(unop_node, Box::new(self.parse()?)))
            },
            Token::RoundBracketOpen => {
                lexer.next_token()?;
                drop(lexer);
                let expr_node = self.parse();
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