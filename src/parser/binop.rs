use crate::ast::lang_ast::binary_operator_node::BinaryOperatorNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::GrammarProductionParsing;

pub struct Binop;

impl Binop {
    pub fn new() -> Binop {
        Binop {

        }
    }
}

impl GrammarProductionParsing<BinaryOperatorNode> for Binop {
    fn parse(&self) -> Result<BinaryOperatorNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();

        let binary_operator_node = match current_token {
            Token::Negation => Ok(BinaryOperatorNode::Subtract),
            Token::Add => Ok(BinaryOperatorNode::Add),
            Token::Multiply => Ok(BinaryOperatorNode::Multiply),
            Token::Divide => Ok(BinaryOperatorNode::Divide),
            Token::Reminder => Ok(BinaryOperatorNode::Remainder),
            Token::BitwiseAnd => Ok(BinaryOperatorNode::BitwiseAnd),
            Token::BitwiseOr => Ok(BinaryOperatorNode::BitwiseOr),
            Token::BitwiseXor => Ok(BinaryOperatorNode::BitwiseXor),
            Token::BitwiseLeftShift => Ok(BinaryOperatorNode::BitwiseLeftShift),
            Token::BitwiseRightShift => Ok(BinaryOperatorNode::BitwiseRightShift),
            Token::Equal => Ok(BinaryOperatorNode::Equal),
            Token::NotEqual => Ok(BinaryOperatorNode::NotEqual),
            Token::LessThan => Ok(BinaryOperatorNode::LessThan),
            Token::LessThanOrEqual => Ok(BinaryOperatorNode::LessThanOrEqual),
            Token::GreaterThan => Ok(BinaryOperatorNode::GreaterThan),
            Token::GreaterThanOrEqual => Ok(BinaryOperatorNode::GreaterThanOrEqual),
            Token::And => Ok(BinaryOperatorNode::And),
            Token::Or => Ok(BinaryOperatorNode::Or),
            Token::Not => Ok(BinaryOperatorNode::Not),
            _ => {
                eprintln!("Error at line {}: unexpected {:?} token", lexer.current_line(), current_token);
                Err(CompilerErrors::SyntaxError)
            }
        };

        lexer.next_token()?;

        binary_operator_node
    }
}