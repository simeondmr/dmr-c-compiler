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
use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;

#[derive(Debug)]
pub enum BlockNode {
    Item(VecDeque<BlockItemNode>)
}

impl GenerateTackyInstructions<()> for BlockNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        let BlockNode::Item(items) = self;
        items.iter().for_each(|item| item.to_tacky(tacky_instructions));
    }
}

impl AstDebugPrinter for BlockNode {
    fn debug_visit(&self) {
        let BlockNode::Item(items) = self;
        items.iter().for_each(|item| item.debug_visit());
    }
}