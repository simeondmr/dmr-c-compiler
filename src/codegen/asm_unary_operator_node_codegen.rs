use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperator;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for AsmUnaryOperator {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            AsmUnaryOperator::Negation => {
                Ok(output_file.write_all("negl ".as_bytes())?)
            },
            AsmUnaryOperator::Not => {
                Ok(output_file.write_all("notl ".as_bytes())?)
            }
        }
    }
}