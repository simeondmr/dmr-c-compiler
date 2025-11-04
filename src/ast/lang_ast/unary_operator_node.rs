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
use crate::tacky::tacky_unary_operator_node::UnaryOperatorTackyNode;

#[derive(Debug)]
pub enum UnaryOperatorNode {
    Complement,
    Negate,
    Not
}

impl GenerateTacky<UnaryOperatorTackyNode> for UnaryOperatorNode {
    fn to_tacky(&self) -> UnaryOperatorTackyNode {
        match self {
            UnaryOperatorNode::Complement => UnaryOperatorTackyNode::Complement,
            UnaryOperatorNode::Negate => UnaryOperatorTackyNode::Negate,
            UnaryOperatorNode::Not => UnaryOperatorTackyNode::Not
        }
    }
}