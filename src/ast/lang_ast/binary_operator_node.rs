use crate::ast::lang_ast::lang_ast_visit_trait::GenerateTacky;
use crate::tacky::binary_operator_tacky_node::BinaryOperatorTackyNode;

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
    BitwiseRightShift
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
            BinaryOperatorNode::BitwiseRightShift => BinaryOperatorTackyNode::BitwiseRightShift
        }
    }
}