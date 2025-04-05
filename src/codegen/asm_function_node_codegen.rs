
use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for FunctionAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        let FunctionAsmNode::FunctionAsmDef(name, ref instructions) = self;
        output_file.write_all(format!(".globl {}\n", name).as_bytes())?;
        output_file.write_all(format!("{}:\n", name).as_bytes())?;
        output_file.write_all("pushq %rbp\n".as_bytes())?;
        output_file.write_all("movq %rsp, %rbp\n".as_bytes())?;
        instructions.iter().try_for_each(|instruction| Ok(instruction.codegen(output_file)?))
    }
}