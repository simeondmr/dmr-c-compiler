use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for AsmBinaryOperatorNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            AsmBinaryOperatorNode::Add =>  Ok(output_file.write_all("addl ".as_bytes())?),
            AsmBinaryOperatorNode::Subtract => Ok(output_file.write_all("subl ".as_bytes())?),
            AsmBinaryOperatorNode::Multiply => Ok(output_file.write_all("imul ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseAnd => Ok(output_file.write_all("and  ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseOr => Ok(output_file.write_all("or ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseXor => Ok(output_file.write_all("xor ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseLeftShift => Ok(output_file.write_all("shl ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseRightShift => Ok(output_file.write_all("shr ".as_bytes())?),
        }
    }
}