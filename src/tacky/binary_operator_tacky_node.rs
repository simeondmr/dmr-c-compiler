use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

#[derive(Debug)]
pub enum BinaryOperatorTackyNode {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift
}

impl GenerateAsm<Option<AsmBinaryOperatorNode>> for BinaryOperatorTackyNode {
    fn to_asm(&self) -> Option<AsmBinaryOperatorNode> {
        match self {
            BinaryOperatorTackyNode::Add => Some(AsmBinaryOperatorNode::Add),
            BinaryOperatorTackyNode::Subtract => Some(AsmBinaryOperatorNode::Subtract),
            BinaryOperatorTackyNode::Multiply => Some(AsmBinaryOperatorNode::Multiply),
            BinaryOperatorTackyNode::BitwiseAnd => Some(AsmBinaryOperatorNode::BitwiseAnd),
            BinaryOperatorTackyNode::BitwiseOr => Some(AsmBinaryOperatorNode::BitwiseOr),
            BinaryOperatorTackyNode::BitwiseXor => Some(AsmBinaryOperatorNode::BitwiseXor),
            BinaryOperatorTackyNode::BitwiseLeftShift => Some(AsmBinaryOperatorNode::BitwiseLeftShift),
            BinaryOperatorTackyNode::BitwiseRightShift => Some(AsmBinaryOperatorNode::BitwiseRightShift),
            // Note that this is an impossible case, because other operators like Multiply and Divide are managed in InstructionTackyNode
            _ => None
        }
    }
}

impl TackyVisitDebug for BinaryOperatorTackyNode {
    fn visit_debug(&self) {
        match self {
            BinaryOperatorTackyNode::Add => println!("Add"),
            BinaryOperatorTackyNode::Subtract => println!("Subtract"),
            BinaryOperatorTackyNode::Multiply => println!("Multiply"),
            BinaryOperatorTackyNode::Divide => println!("Divide"),
            BinaryOperatorTackyNode::Remainder => println!("Remainder"),
            BinaryOperatorTackyNode::BitwiseAnd => println!("BitwiseAnd"),
            BinaryOperatorTackyNode::BitwiseOr => println!("BitwiseOr"),
            BinaryOperatorTackyNode::BitwiseXor => println!("BitwiseXor"),
            BinaryOperatorTackyNode::BitwiseLeftShift => println!("BitwiseLeftShift"),
            BinaryOperatorTackyNode::BitwiseRightShift => println!("BitwiseRightShift")
        }
    }
}
