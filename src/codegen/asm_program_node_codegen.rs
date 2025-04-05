use std::fs::File;
use std::io::Error;
use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for AsmProgramNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        Ok(function.codegen(output_file)?)
    }
}
