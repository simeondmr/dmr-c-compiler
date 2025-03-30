use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for AsmProgramNode {
    fn codegen(&self) {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        function.codegen();
    }
}
