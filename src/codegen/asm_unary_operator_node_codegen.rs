use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperator;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for AsmUnaryOperator {
    fn codegen(&self) {
        match self {
            AsmUnaryOperator::Negation => print!("negl "),
            AsmUnaryOperator::Not => print!("notl ")
        }
    }
}