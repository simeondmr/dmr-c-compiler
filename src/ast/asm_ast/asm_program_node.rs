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

use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::ast::lang_ast::lang_ast_visit_trait::AstDebugPrinter;

pub enum AsmProgramNode {
    ProgramAsmDef(Vec<FunctionAsmNode>)
}

impl AstDebugPrinter for AsmProgramNode {
    fn debug_visit(&self) {
        let AsmProgramNode::ProgramAsmDef(functions) = self;
        println!("Program(");
        functions.into_iter().for_each(|function| { function.debug_visit(); });
        println!(")");
    }
}