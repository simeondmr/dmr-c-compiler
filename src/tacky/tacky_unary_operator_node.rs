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

use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperatorNode;
use crate::ast::lang_ast::lang_ast_visit_trait::AstDebugPrinter;
use crate::tacky::tacky_visit_trait::GenerateAsm;

pub enum UnaryOperatorTackyNode {
    Complement,
    Negate,
    Not
}

impl GenerateAsm<AsmUnaryOperatorNode> for UnaryOperatorTackyNode {
    fn to_asm(&self) -> AsmUnaryOperatorNode {
        match self {
            UnaryOperatorTackyNode::Complement => AsmUnaryOperatorNode::Not,
            UnaryOperatorTackyNode::Negate => AsmUnaryOperatorNode::Negation,
            // Note: UnaryOperatorTackyNode::Not will be not translated into an AsmUnaryOperatorNode so I put an Empty Node for it
            UnaryOperatorTackyNode::Not => AsmUnaryOperatorNode::Empty
        }
    }
}

impl AstDebugPrinter for UnaryOperatorTackyNode {
    fn debug_visit(&self) {
        match self {
            UnaryOperatorTackyNode::Complement => println!("Complement"),
            UnaryOperatorTackyNode::Negate => println!("Negate"),
            UnaryOperatorTackyNode::Not => println!("Not")
        }
    }
}