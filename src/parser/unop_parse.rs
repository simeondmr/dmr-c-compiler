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

use crate::ast::lang_ast::unary_operator_node::UnaryOperatorNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::program_parse::GrammarProductionParsing;

pub struct UnopParse;

impl UnopParse {
    pub fn new() -> UnopParse {
        UnopParse {

        }
    }
}

impl GrammarProductionParsing<UnaryOperatorNode> for UnopParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<UnaryOperatorNode, CompilerErrors> {
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