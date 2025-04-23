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