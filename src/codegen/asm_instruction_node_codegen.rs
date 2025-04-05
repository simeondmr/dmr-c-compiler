use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for InstructionAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            InstructionAsmNode::Mov(src_operand, dest_operand) => {
                output_file.write_all("movl ".as_bytes())?;
                src_operand.codegen(output_file)?;
                output_file.write_all(", ".as_bytes())?;
                dest_operand.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Unary(operator, operand) => {
                operator.codegen(output_file)?;
                operand.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::AllocateStack(stack_offet) => {
                Ok(output_file.write_all(format!("subq {}, %rsp\n", *stack_offet).as_bytes())?)
            },
            InstructionAsmNode::Ret => {
                output_file.write_all("movq %rbp, %rsp\n".as_bytes())?;
                output_file.write_all("popq %rbp\n".as_bytes())?;
                Ok(output_file.write_all("ret\n".as_bytes())?)
            }
        }
    }
}