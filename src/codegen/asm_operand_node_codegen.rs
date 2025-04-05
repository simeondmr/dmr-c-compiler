use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for OperandAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error>{
        match self {
            OperandAsmNode::Imm(value) => {
                Ok(output_file.write_all(format!("${}", value).as_bytes())?)
            },
            OperandAsmNode::Register(reg) => reg.codegen(output_file),
            OperandAsmNode::Pseudo(value) => {
                Ok(output_file.write_all(format!("t{}", value).as_bytes())?)
            },
            OperandAsmNode::Stack(value) => {
                Ok(output_file.write_all(format!("{}(%rsp)", value).as_bytes())?)
            }
        }
    }
}