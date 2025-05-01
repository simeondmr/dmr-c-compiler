use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for AsmBinaryOperatorNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            AsmBinaryOperatorNode::Add =>  Ok(output_file.write_all("\taddl ".as_bytes())?),
            AsmBinaryOperatorNode::Subtract => Ok(output_file.write_all("\tsubl ".as_bytes())?),
            AsmBinaryOperatorNode::Multiply => Ok(output_file.write_all("\timull ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseAnd => Ok(output_file.write_all("\tandl  ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseOr => Ok(output_file.write_all("\torl ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseXor => Ok(output_file.write_all("\txorl ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseLeftShift => Ok(output_file.write_all("\tshll ".as_bytes())?),
            AsmBinaryOperatorNode::BitwiseRightShift => Ok(output_file.write_all("\tshrl ".as_bytes())?),
            // Note: operators like AsmBinaryOperatorNode::Equal, AsmBinaryOperatorNode::NotEqual etc, obliviously haven't a direct translaction instruction like operators above 
            _ =>  Ok(())
        }
    }
}