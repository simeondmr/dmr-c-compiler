use crate::ast::lang_ast::lang_ast_visit_trait::GenerateTacky;
use crate::tacky::tacky_unary_operator_node::UnaryOperatorTackyNode;

#[derive(Debug)]
pub enum UnaryOperatorNode {
    Complement,
    Negate,
    Not
}

impl GenerateTacky<UnaryOperatorTackyNode> for UnaryOperatorNode {
    fn to_tacky(&self) -> UnaryOperatorTackyNode {
        match self {
            UnaryOperatorNode::Complement => UnaryOperatorTackyNode::Complement,
            UnaryOperatorNode::Negate => UnaryOperatorTackyNode::Negate,
            UnaryOperatorNode::Not => UnaryOperatorTackyNode::Not
        }
    }
}