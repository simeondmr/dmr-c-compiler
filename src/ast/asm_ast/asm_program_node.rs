use crate::ast::asm_ast::asm_ast_visit_trait::{AsmReplacingPseudoregisters, AstAsmDebugPrinter, FixingInstruction};
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::codegen::stack_alloc_table::StackAllocTable;

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

impl AsmReplacingPseudoregisters for AsmProgramNode {
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32 {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        function.replacing_pseudoregisters(stack_alloc_table)
    }
}

impl FixingInstruction for AsmProgramNode {
    fn fixing_instructions(&mut self, stack_offset: i32) {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        function.fixing_instructions(stack_offset);
    }
}