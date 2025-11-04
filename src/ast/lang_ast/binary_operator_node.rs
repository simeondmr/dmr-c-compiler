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

use crate::ast::lang_ast::lang_ast_visit_trait::GenerateTacky;
use crate::tacky::binary_operator_tacky_node::BinaryOperatorTackyNode;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BinaryOperatorNode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
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
    GreaterThanOrEqual,
    Not,
    And,
    Or,
    Comma
}

impl GenerateTacky<BinaryOperatorTackyNode> for BinaryOperatorNode {
    fn to_tacky(&self) -> BinaryOperatorTackyNode {
        match self {
            BinaryOperatorNode::Add => BinaryOperatorTackyNode::Add,
            BinaryOperatorNode::Subtract => BinaryOperatorTackyNode::Subtract,
            BinaryOperatorNode::Multiply => BinaryOperatorTackyNode::Multiply,
            BinaryOperatorNode::Divide => BinaryOperatorTackyNode::Divide,
            BinaryOperatorNode::Remainder => BinaryOperatorTackyNode::Remainder,
            BinaryOperatorNode::BitwiseAnd => BinaryOperatorTackyNode::BitwiseAnd,
            BinaryOperatorNode::BitwiseOr => BinaryOperatorTackyNode::BitwiseOr,
            BinaryOperatorNode::BitwiseXor => BinaryOperatorTackyNode::BitwiseXor,
            BinaryOperatorNode::BitwiseLeftShift => BinaryOperatorTackyNode::BitwiseLeftShift,
            BinaryOperatorNode::BitwiseRightShift => BinaryOperatorTackyNode::BitwiseRightShift,
            BinaryOperatorNode::LessThan => BinaryOperatorTackyNode::LessThan,
            BinaryOperatorNode::LessThanOrEqual => BinaryOperatorTackyNode::LessThanOrEqual,
            BinaryOperatorNode::GreaterThan => BinaryOperatorTackyNode::GreaterThan,
            BinaryOperatorNode::GreaterThanOrEqual => BinaryOperatorTackyNode::GreaterThanOrEqual,
            BinaryOperatorNode::Equal => BinaryOperatorTackyNode::Equal,
            BinaryOperatorNode::NotEqual => BinaryOperatorTackyNode::NotEqual,
            // Note: operators like &&, ||, ! obliviously cannot be converted into a TackyNode, so for covering them I decided to put an Empty Node
            _ => BinaryOperatorTackyNode::Empty
        }
    }
}