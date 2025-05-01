use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum AsmUnaryOperatorNode {
    Empty,
    Negation,
    Not
}

impl AstAsmDebugPrinter for AsmUnaryOperatorNode {
    fn debug_visit(&self) {
        match self {
            AsmUnaryOperatorNode::Empty => print!("Empty"),
            AsmUnaryOperatorNode::Negation => print!("Negation "),
            AsmUnaryOperatorNode::Not => print!("Not ")
        }
    }
}