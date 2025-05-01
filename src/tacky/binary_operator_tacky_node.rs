use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::ast::asm_ast::asm_instruction_node::ConditionCode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

#[derive(Debug)]
pub enum BinaryOperatorTackyNode {
    Empty,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLeftShift,
    BitwiseRightShift,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
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
            BinaryOperatorTackyNode::Equal => Some(AsmBinaryOperatorNode::Equal),
            BinaryOperatorTackyNode::NotEqual => Some(AsmBinaryOperatorNode::NotEqual),
            BinaryOperatorTackyNode::LessThan => Some(AsmBinaryOperatorNode::LessThan),
            BinaryOperatorTackyNode::LessThanOrEqual => Some(AsmBinaryOperatorNode::LessThanOrEqual),
            BinaryOperatorTackyNode::GreaterThan => Some(AsmBinaryOperatorNode::GreaterThan),
            BinaryOperatorTackyNode::GreaterThanOrEqual => Some(AsmBinaryOperatorNode::GreaterThanOrEqual),
            // Note that this is an impossible case, because other operators like Multiply and Divide are managed in InstructionTackyNode
            _ => None
        }
    }
}

impl BinaryOperatorTackyNode {
    pub fn to_condition_code(&self) -> Option<ConditionCode> {
        match self {
            BinaryOperatorTackyNode::Empty => None,
            BinaryOperatorTackyNode::Add => None,
            BinaryOperatorTackyNode::Subtract => None,
            BinaryOperatorTackyNode::Multiply => None,
            BinaryOperatorTackyNode::Divide => None,
            BinaryOperatorTackyNode::Remainder => None,
            BinaryOperatorTackyNode::BitwiseAnd => None,
            BinaryOperatorTackyNode::BitwiseOr => None,
            BinaryOperatorTackyNode::BitwiseXor => None,
            BinaryOperatorTackyNode::BitwiseLeftShift => None,
            BinaryOperatorTackyNode::BitwiseRightShift => None,
            BinaryOperatorTackyNode::Equal => Some(ConditionCode::E),
            BinaryOperatorTackyNode::NotEqual => Some(ConditionCode::Ne),
            BinaryOperatorTackyNode::LessThan => Some(ConditionCode::L),
            BinaryOperatorTackyNode::LessThanOrEqual => Some(ConditionCode::Le),
            BinaryOperatorTackyNode::GreaterThan => Some(ConditionCode::G),
            BinaryOperatorTackyNode::GreaterThanOrEqual => Some(ConditionCode::Ge)
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
            BinaryOperatorTackyNode::BitwiseRightShift => println!("BitwiseRightShift"),
            BinaryOperatorTackyNode::LessThan => println!("LessThan"),
            BinaryOperatorTackyNode::LessThanOrEqual => println!("LessThanOrEqual"),
            BinaryOperatorTackyNode::GreaterThan => println!("GreaterThan"),
            BinaryOperatorTackyNode::GreaterThanOrEqual => println!("GreaterThanOrEqual"),
            BinaryOperatorTackyNode::Equal => println!("Equal"),
            BinaryOperatorTackyNode::NotEqual => println!("NotEqual"),
            // Note: I decided to as an "Empty" BinaryOperatorTackyNode
            BinaryOperatorTackyNode::Empty => println!("Empty")
        }
    }
}
