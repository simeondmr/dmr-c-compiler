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

use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum AsmBinaryOperatorNode {
    Add,
    Subtract,
    Multiply,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual
}

impl AstAsmDebugPrinter for AsmBinaryOperatorNode {
    fn debug_visit(&self) {
        match self {
            AsmBinaryOperatorNode::Add => print!("Add "),
            AsmBinaryOperatorNode::Subtract => println!("Subtract"),
            AsmBinaryOperatorNode::Multiply => println!("Multiply"),
            AsmBinaryOperatorNode::BitwiseAnd => println!("BitwiseAnd"),
            AsmBinaryOperatorNode::BitwiseOr => println!("BitwiseOr"),
            AsmBinaryOperatorNode::BitwiseXor => println!("BitwiseXor"),
            AsmBinaryOperatorNode::BitwiseLeftShift => println!("BitwiseLeftShift"),
            AsmBinaryOperatorNode::BitwiseRightShift => println!("BitwiseRightShift"),
            AsmBinaryOperatorNode::Equal => println!("Equal"),
            AsmBinaryOperatorNode::NotEqual => println!("NotEqual"),
            AsmBinaryOperatorNode::LessThan => println!("LessThan"),
            AsmBinaryOperatorNode::LessThanOrEqual => println!("LessThanOrEqual"),
            AsmBinaryOperatorNode::GreaterThan => println!("GreaterThan"),
            AsmBinaryOperatorNode::GreaterThanOrEqual => println!("GreaterThanOrEqual"),
        }
    }
}