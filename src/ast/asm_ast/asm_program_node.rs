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
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;

pub enum AsmProgramNode {
    ProgramAsmDef(FunctionAsmNode)
}

impl AstAsmDebugPrinter for AsmProgramNode {
    fn debug_visit(&self) {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        println!("Program(");
        function.debug_visit();
        println!(")");
    }
}