use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for InstructionAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            InstructionAsmNode::Mov { src, dest} => {
                output_file.write_all("\tmovl ".as_bytes())?;
                src.codegen(output_file)?;
                output_file.write_all(", ".as_bytes())?;
                dest.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Unary { operator, operand} => {
                operator.codegen(output_file)?;
                operand.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Binary { operator, src, dest } => {
                operator.codegen(output_file)?;
                src.codegen(output_file)?;
                output_file.write_all(", ".as_bytes())?;
                dest.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)

            },
            InstructionAsmNode::Idiv(operator) => {
                output_file.write_all("\tidivl ".as_bytes())?;
                operator.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Cdq => Ok(output_file.write_all("\tcdq\n".as_bytes())?),
            InstructionAsmNode::AllocateStack(stack_offet) => Ok(output_file.write_all(format!("\tsubq ${}, %rsp\n", stack_offet.abs()).as_bytes())?),
            InstructionAsmNode::Ret => {
                output_file.write_all("\tmovq %rbp, %rsp\n".as_bytes())?;
                output_file.write_all("\tpopq %rbp\n".as_bytes())?;
                Ok(output_file.write_all("\tret\n".as_bytes())?)
            }
        }
    }
}