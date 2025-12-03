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

use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::ast::lang_ast::lang_ast_visit_trait::AstDebugPrinter;
use crate::tacky::tacky_function_node::FunctionTackyNode;
use crate::tacky::tacky_visit_trait::GenerateAsm;

pub enum ProgramTackyNode {
    ProgramDef(Vec<FunctionTackyNode>)
}

impl AstDebugPrinter for ProgramTackyNode {
    fn debug_visit(&self) {
        let ProgramTackyNode::ProgramDef(functions) = self;
        functions.into_iter().for_each(|function| function.debug_visit());
    }
}

impl GenerateAsm<AsmProgramNode> for ProgramTackyNode {
    fn to_asm(&self) -> AsmProgramNode {
        let ProgramTackyNode::ProgramDef(functions_tacky_node) = self;
        let mut asm_functions = Vec::new();
        functions_tacky_node.into_iter().for_each(|function_tacky_node| asm_functions.push(function_tacky_node.to_asm()));
        AsmProgramNode::ProgramAsmDef(asm_functions)
    }
}