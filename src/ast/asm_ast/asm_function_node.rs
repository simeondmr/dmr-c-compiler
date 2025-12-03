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
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::lang_ast::lang_ast_visit_trait::AstDebugPrinter;

/// This enum provides a Node for defining functions node
pub enum FunctionAsmNode {
    FunctionAsmDef {
        func_name: String,
        stack_alloc_size: i32,
        asm_instructions: VecDeque<InstructionAsmNode>
    }
}

impl AstDebugPrinter for FunctionAsmNode {
    fn debug_visit(&self) {
        let FunctionAsmNode::FunctionAsmDef { func_name, stack_alloc_size, ref asm_instructions} = self;
        println!("Function(\nname = {func_name}, stack_alloc: {} bytes", stack_alloc_size.abs() );
        asm_instructions.iter().for_each(|instruction| instruction.debug_visit());
        println!(")");
    }
}