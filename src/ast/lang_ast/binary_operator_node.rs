use crate::ast::lang_ast::lang_ast_visit_trait::GenerateTacky;
use crate::tacky::binary_operator_tacky_node::BinaryOperatorTackyNode;

#[allow(dead_code)]
#[derive(Debug)]
pub enum BinaryOperatorNode {
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
    Not,
    And,
    Or,
}

impl GenerateTacky<BinaryOperatorTackyNode> for BinaryOperatorNode {
    fn to_tacky(&self) -> BinaryOperatorTackyNode {
        match self {
            BinaryOperatorNode::Add => BinaryOperatorTackyNode::Add,
            BinaryOperatorNode::Subtract => BinaryOperatorTackyNode::Subtract,
            BinaryOperatorNode::Multiply => BinaryOperatorTackyNode::Multiply,
            BinaryOperatorNode::Divide => BinaryOperatorTackyNode::Divide,
            BinaryOperatorNode::Remainder => BinaryOperatorTackyNode::Remainder,
            BinaryOperatorNode::BitwiseAnd => BinaryOperatorTackyNode::BitwiseAnd,
            BinaryOperatorNode::BitwiseOr => BinaryOperatorTackyNode::BitwiseOr,
            BinaryOperatorNode::BitwiseXor => BinaryOperatorTackyNode::BitwiseXor,
            BinaryOperatorNode::BitwiseLeftShift => BinaryOperatorTackyNode::BitwiseLeftShift,
            BinaryOperatorNode::BitwiseRightShift => BinaryOperatorTackyNode::BitwiseRightShift,
            BinaryOperatorNode::LessThan => BinaryOperatorTackyNode::LessThan,
            BinaryOperatorNode::LessThanOrEqual => BinaryOperatorTackyNode::LessThanOrEqual,
            BinaryOperatorNode::GreaterThan => BinaryOperatorTackyNode::GreaterThan,
            BinaryOperatorNode::GreaterThanOrEqual => BinaryOperatorTackyNode::GreaterThanOrEqual,
            BinaryOperatorNode::Equal => BinaryOperatorTackyNode::Equal,
            BinaryOperatorNode::NotEqual => BinaryOperatorTackyNode::NotEqual,
            // Note: operators like &&, ||, ! obliviously cannot be converted into a TackyNode, so for covering them I decided to put an Empty Node
            _ => BinaryOperatorTackyNode::Empty
        }
    }
}