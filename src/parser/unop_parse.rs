use crate::ast::lang_ast::unary_operator_node::UnaryOperatorNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program_parse::GrammarProductionParsing;

pub struct UnopParse;

impl UnopParse {
    pub fn new() -> UnopParse {
        UnopParse {

        }
    }
}

impl GrammarProductionParsing<UnaryOperatorNode> for UnopParse {
    fn parse(&self) -> Result<UnaryOperatorNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();
        if let Token::BitwiseComplement = current_token {
            lexer.next_token()?;
            Ok(UnaryOperatorNode::Complement)
        } else if let Token::Negation = current_token {
            lexer.next_token()?;
            Ok(UnaryOperatorNode::Negate)
        }  else if let Token::Not = current_token {
            lexer.next_token()?;
            Ok(UnaryOperatorNode::Not)
        } else {
            eprintln!("Error at line {}: unexpected {:?} token", lexer.current_line(), current_token);
            Err(CompilerErrors::SyntaxError)
        }
    }
}