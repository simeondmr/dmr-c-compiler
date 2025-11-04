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

use std::collections::VecDeque;
use crate::ast::lang_ast::block_node::BlockNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::block_item_parse::BlockItemParse;
use crate::parser::program_parse::GrammarProductionParsing;

pub struct BlockParse {
    block_item: BlockItemParse
}

impl BlockParse {
    pub fn new() -> BlockParse {
        BlockParse {
            block_item: BlockItemParse::new()
        }
    }
}

impl GrammarProductionParsing<BlockNode> for BlockParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<BlockNode, CompilerErrors> {
        Self::match_token(&Token::CurlyBracketOpen, lexer)?;
        let mut list_block_item = VecDeque::new();
        while lexer.current_token() != Token::CurlyBracketClose {
            let current_block_item = self.block_item.parse(lexer)?;
            list_block_item.push_back(current_block_item);
        }
        lexer.next_token()?;
        Ok(BlockNode::Item(list_block_item))
    }
}