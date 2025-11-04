// dmr C compiler
// Copyright (C) 2025  Simeon Tornabene
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, see <https://www.gnu.org/licenses/>.

use crate::ast::lang_ast::binary_operator_node::BinaryOperatorNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::program_parse::GrammarProductionParsing;

pub struct BinopParse;

impl BinopParse {
    pub fn new() -> BinopParse {
        BinopParse {

        }
    }
}

impl GrammarProductionParsing<BinaryOperatorNode> for BinopParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<BinaryOperatorNode, CompilerErrors> {
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
            Token::Comma => Ok(BinaryOperatorNode::Comma),
            _ => {
                eprintln!("Error at line {}: unexpected {:?} token", lexer.current_line(), current_token);
                Err(CompilerErrors::SyntaxError)
            }
        };
        lexer.next_token()?;
        binary_operator_node
    }
}