use crate::ast::asm_ast::asm_ast_visit_trait::FixingInstruction;
use crate::ast::asm_ast::asm_program_node::AsmProgramNode;

impl FixingInstruction for AsmProgramNode {
    fn fixing_instructions(&mut self, stack_offset: i32) {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        function.fixing_instructions(stack_offset);
    }
}