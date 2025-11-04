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
pub enum AsmUnaryOperatorNode {
    Empty,
    Negation,
    Not
}

impl AstAsmDebugPrinter for AsmUnaryOperatorNode {
    fn debug_visit(&self) {
        match self {
            AsmUnaryOperatorNode::Empty => print!("Empty"),
            AsmUnaryOperatorNode::Negation => print!("Negation "),
            AsmUnaryOperatorNode::Not => print!("Not ")
        }
    }
}