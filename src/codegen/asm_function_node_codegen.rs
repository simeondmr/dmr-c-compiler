
use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for FunctionAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        let FunctionAsmNode::FunctionAsmDef { func_name, ref asm_instructions } = self;
        output_file.write_all(format!(".globl {}\n", func_name).as_bytes())?;
        output_file.write_all(format!("{}:\n", func_name).as_bytes())?;
        output_file.write_all("\tpushq %rbp\n".as_bytes())?;
        output_file.write_all("\tmovq %rsp, %rbp\n".as_bytes())?;
        asm_instructions.iter().try_for_each(|instruction| Ok(instruction.codegen(output_file)?))
    }
}