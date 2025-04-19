use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperatorNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

pub enum UnaryOperatorTackyNode {
    Complement,
    Negate
}

impl TackyVisitDebug for UnaryOperatorTackyNode {
    fn visit_debug(&self) {
        match self {
            UnaryOperatorTackyNode::Complement => println!("Complement"),
            UnaryOperatorTackyNode::Negate => println!("Negate")
        }
    }
}

impl GenerateAsm<AsmUnaryOperatorNode> for UnaryOperatorTackyNode {
    fn to_asm(&self) -> AsmUnaryOperatorNode {
        match self {
            UnaryOperatorTackyNode::Complement => AsmUnaryOperatorNode::Not,
            UnaryOperatorTackyNode::Negate => AsmUnaryOperatorNode::Negation
        }
    }
}