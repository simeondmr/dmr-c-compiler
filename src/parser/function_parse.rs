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

use crate::ast::lang_ast::function_node::FunctionNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::block_parse::BlockParse;
use crate::parser::program_parse::{GrammarProductionParsing};

pub struct FunctionParse {
    block: BlockParse
}

impl FunctionParse {
    pub fn new() -> FunctionParse {
        FunctionParse {
            block: BlockParse::new()
        }
    }
}

impl GrammarProductionParsing<FunctionNode> for FunctionParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<FunctionNode, CompilerErrors> {
        Self::match_token(&Token::Int, lexer)?;
        let current_token = lexer.current_token();
        Self::match_token(&Token::Literal("".to_string()), lexer)?;
        let func_name = current_token.extract_literal_val().unwrap();
        Self::match_token(&Token::RoundBracketOpen, lexer)?;
        Self::match_token(&Token::Void, lexer)?;
        Self::match_token(&Token::RoundBracketClose, lexer)?;
        Ok(FunctionNode::FunctionDef { func_name, block: self.block.parse(lexer)? })
    }
}