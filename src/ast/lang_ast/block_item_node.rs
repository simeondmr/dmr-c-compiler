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
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::ast::lang_ast::statement_node::StatementNode;
use crate::tacky::tacky_instruction_node::InstructionTackyNode;

#[derive(Debug)]
pub enum BlockItemNode {
    Statement(StatementNode),
    Declaration(DeclarationNode)
}

impl GenerateTackyInstructions<()> for BlockItemNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        if let BlockItemNode::Statement(statement_node) = self {
            statement_node.to_tacky(tacky_instructions)
        } else if let BlockItemNode::Declaration(declaration_node) = self {
            declaration_node.to_tacky(tacky_instructions);
        }
    }
}

impl AstDebugPrinter for BlockItemNode {
    fn debug_visit(&self) {
        if let BlockItemNode::Statement(statement_node) = self {
            statement_node.debug_visit();
        } else if let BlockItemNode::Declaration(declaration_node) = self {
            declaration_node.debug_visit();
        }
    }
}