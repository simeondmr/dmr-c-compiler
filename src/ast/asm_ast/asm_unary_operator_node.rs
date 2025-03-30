use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum AsmUnaryOperator {
    Negation,
    Not
}

impl AstAsmDebugPrinter for AsmUnaryOperator {
    fn debug_visit(&self) {
        match self {
            AsmUnaryOperator::Negation => print!("Negation "),
            AsmUnaryOperator::Not => print!("Not ")
        }
    }
}