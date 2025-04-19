use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum AsmUnaryOperatorNode {
    Negation,
    Not
}

impl AstAsmDebugPrinter for AsmUnaryOperatorNode {
    fn debug_visit(&self) {
        match self {
            AsmUnaryOperatorNode::Negation => print!("Negation "),
            AsmUnaryOperatorNode::Not => print!("Not ")
        }
    }
}