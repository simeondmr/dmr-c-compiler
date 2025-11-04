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

use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::expr_parse::ExprParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct VarDeclarationParse {

}

impl VarDeclarationParse {
    pub fn new() -> VarDeclarationParse {
        VarDeclarationParse {

        }
    }
}

impl GrammarProductionParsing<DeclarationNode> for VarDeclarationParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<DeclarationNode, CompilerErrors> {
        Self::match_token(&Token::Int, lexer)?;
        let var_name = lexer.current_token();
        Self::match_token(&Token::Literal("".to_string()), lexer)?;
        let assignment_token = lexer.current_token();
        let mut init: Option<ExprNode> = None;
        if let Token::Assignment = assignment_token {
            Self::match_token(&Token::Assignment, lexer)?;
            init = Some(ExprParse::new().parse(lexer, 0, false)?);
        }
        Self::match_token(&Token::Semicolon, lexer)?;
        // Note: During the parsing stage put 0 as var_name_index for every variable. During variable resolution pass the field identifier_index will be fixed with the correct value
        Ok(DeclarationNode::VariableDeclaration { var_name: var_name.extract_literal_val().unwrap(), var_name_index: 0, init })
    }
}