use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum AsmBinaryOperatorNode {
    Add,
    Subtract,
    Multiply,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift
}

impl AstAsmDebugPrinter for AsmBinaryOperatorNode {
    fn debug_visit(&self) {
        match self {
            AsmBinaryOperatorNode::Add => print!("Add "),
            AsmBinaryOperatorNode::Subtract => println!("Subtract"),
            AsmBinaryOperatorNode::Multiply => println!("Multiply"),
            AsmBinaryOperatorNode::BitwiseAnd => println!("BitwiseAnd"),
            AsmBinaryOperatorNode::BitwiseOr => println!("BitwiseOr"),
            AsmBinaryOperatorNode::BitwiseXor => println!("BitwiseXor"),
            AsmBinaryOperatorNode::BitwiseLeftShift => println!("BitwiseLeftShift"),
            AsmBinaryOperatorNode::BitwiseRightShift => println!("BitwiseRightShift"),
        }
    }
}