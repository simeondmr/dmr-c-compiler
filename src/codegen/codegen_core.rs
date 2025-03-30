use crate::ast::asm_ast::asm_ast_visit_trait::{AsmReplacingPseudoregisters, FixingInstruction};
use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::codegen::stack_alloc_table::StackAllocTable;
use crate::codegen::asm_codegen_trait::Codegen;

pub struct CodegenCore;

impl CodegenCore {
    pub fn new() -> CodegenCore {
        CodegenCore {

        }
    }

    pub fn codegen(&self, asm_ast: &mut AsmProgramNode) {
        let mut stack_alloc_table = StackAllocTable::new();
        let stack_offset = asm_ast.replacing_pseudoregisters(&mut stack_alloc_table);
        asm_ast.fixing_instructions(stack_offset);
        asm_ast.codegen();
    }
}