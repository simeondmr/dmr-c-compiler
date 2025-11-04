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
use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;

/// This enum provides a Node for defining functions node
pub enum FunctionAsmNode {
    FunctionAsmDef {
        func_name: String,
        asm_instructions: VecDeque<InstructionAsmNode>
    }
}

impl AstAsmDebugPrinter for FunctionAsmNode {
    fn debug_visit(&self) {
        let FunctionAsmNode::FunctionAsmDef { func_name, ref asm_instructions} = self;
        println!("Function(\nname = {}", func_name);
        asm_instructions.iter().for_each(|instruction| instruction.debug_visit());
        println!(")");
    }
}