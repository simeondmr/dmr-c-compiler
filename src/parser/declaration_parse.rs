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
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::function_declaration_parse::FunctionDeclarationParse;
use crate::parser::program_parse::GrammarProductionParsing;
use crate::parser::var_declaration_parse::VarDeclarationParse;

pub struct DeclarationParse;

impl DeclarationParse {
    pub fn new() -> DeclarationParse {
        DeclarationParse {
            
        }
    }
}

impl GrammarProductionParsing<DeclarationNode> for DeclarationParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<DeclarationNode, CompilerErrors> {
        //NOTE: as you can can se in \grammar\grammar.txt, the production <declaration> is ambiguous, so one more lookahead is necessary for the grammar production dispaccer
        lexer.feed_lookaheads_tokens(2)?;
        if lexer.current_token() == Token::Int && lexer.match_lookahead(0, &Token::Literal("".to_string()))? && lexer.match_lookahead(1, &Token::RoundBracketOpen)? {
            Ok(DeclarationNode::FunctionDeclaration(FunctionDeclarationParse.parse(lexer)?))
        } else {
            VarDeclarationParse::new().parse(lexer)
        }
    }
}