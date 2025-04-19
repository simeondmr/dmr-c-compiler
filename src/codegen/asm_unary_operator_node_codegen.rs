use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperatorNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for AsmUnaryOperatorNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            AsmUnaryOperatorNode::Negation => Ok(output_file.write_all("negl ".as_bytes())?),
            AsmUnaryOperatorNode::Not => Ok(output_file.write_all("notl ".as_bytes())?)
        }
    }
}