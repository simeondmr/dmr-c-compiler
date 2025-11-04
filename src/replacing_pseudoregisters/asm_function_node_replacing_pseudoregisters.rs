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

use crate::ast::asm_ast::asm_ast_visit_trait::AsmReplacingPseudoregisters;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::codegen::stack_alloc_table::StackAllocTable;

impl AsmReplacingPseudoregisters for FunctionAsmNode {
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32 {
        let FunctionAsmNode::FunctionAsmDef { func_name: _, ref mut asm_instructions } = self;
        asm_instructions.iter_mut().for_each(|instruction| { instruction.replacing_pseudoregisters(stack_alloc_table); });
        stack_alloc_table.stack_offset()
    }
}