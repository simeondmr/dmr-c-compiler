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

use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::declaration_parse::DeclarationParse;
use crate::parser::program_parse::GrammarProductionParsing;
use crate::parser::statement_parse::Statement;

pub struct BlockItemParse;

impl BlockItemParse {
    pub fn new() -> BlockItemParse {
        BlockItemParse {
            
        }
    }
}

impl GrammarProductionParsing<BlockItemNode> for BlockItemParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<BlockItemNode, CompilerErrors> {
        let current_token = lexer.current_token();
        if let Token::Int = current_token {
            Ok(BlockItemNode::Declaration(DeclarationParse::new().parse(lexer)?))
        } else {
            Ok(BlockItemNode::Statement(Statement::new().parse(lexer)?))
        }
    }
}