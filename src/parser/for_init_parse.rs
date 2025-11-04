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

use crate::ast::lang_ast::statement_node::ForInit;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::expr_parse::ExprParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};
use crate::parser::var_declaration_parse::VarDeclarationParse;

pub struct ForInitParse;

impl ForInitParse {
    pub fn new() -> ForInitParse {
        ForInitParse {

        }
    }
}

impl GrammarProductionParsing<ForInit> for ForInitParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<ForInit, CompilerErrors> {
        let current_token = lexer.current_token();
        let result = Ok(if let Token::Int = current_token {
            ForInit::DeclarationInit(VarDeclarationParse::new().parse(lexer)?)
        } else if let Token::Semicolon = current_token {
            lexer.next_token()?;
            ForInit::ExpressionInit(None)
        } else {
            let expr_node = ForInit::ExpressionInit(Some(ExprParse::new().parse(lexer, 0, false)?));
            Self::match_token(&Token::Semicolon, lexer)?;
            expr_node
        });
        result
    }
}
