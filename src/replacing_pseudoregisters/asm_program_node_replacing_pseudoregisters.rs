use crate::ast::asm_ast::asm_ast_visit_trait::AsmReplacingPseudoregisters;
use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::codegen::stack_alloc_table::StackAllocTable;

impl AsmReplacingPseudoregisters for AsmProgramNode {
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32 {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        function.replacing_pseudoregisters(stack_alloc_table)
    }
}