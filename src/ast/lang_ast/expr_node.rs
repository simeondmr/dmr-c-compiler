use crate::ast::lang_ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::lang_ast::expr_node::ExprNode::{Binary, Constant, Unary};
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky, GenerateTackyInstructions};
use crate::ast::lang_ast::unary_operator_node::UnaryOperatorNode;
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_val_node::{TemporaryVar, ValTackyNode};

#[derive(Debug)]
pub enum ExprNode {
    Constant(i32),
    Unary {
        unary_operator: UnaryOperatorNode,
        expr: Box<ExprNode>
    },
    Binary {
        binary_operator: BinaryOperatorNode,
        left_expr: Box<ExprNode>,
        right_expr: Box<ExprNode>
    }
}

impl GenerateTackyInstructions<ValTackyNode> for ExprNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> ValTackyNode {
        match self {
            Constant(value) => ValTackyNode::Constant(*value),
            Unary { unary_operator , expr } => {
                let src = expr.to_tacky(tacky_instructions);
                let dest = ValTackyNode::Var(TemporaryVar::generate());
                tacky_instructions.push(InstructionTackyNode::Unary { unary_operator: unary_operator.to_tacky(), src, dest: dest.clone() });
                dest
            },
            Binary { binary_operator, left_expr, right_expr } => {
                let left_exp_tacky_node = left_expr.to_tacky(tacky_instructions);
                let right_exp_tacky_node = right_expr.to_tacky(tacky_instructions);
                let dest = ValTackyNode::Var(TemporaryVar::generate());
                tacky_instructions.push(InstructionTackyNode::Binary { binary_operator: binary_operator.to_tacky(), left_expr: left_exp_tacky_node, right_expr: right_exp_tacky_node, dest: dest.clone() });
                dest
            }
        }
    }
}

impl AstDebugPrinter for ExprNode {
    fn debug_visit(&self) {
        match self {
            Constant(value) => println!("Constant({})", value),
            Unary { unary_operator, expr } => {
                println!("Unary {:?} (", unary_operator);
                expr.debug_visit();
                println!(")");
            },
            Binary { binary_operator, left_expr, right_expr } => {
                println!("Binary {:?} (", binary_operator);
                left_expr.debug_visit();
                right_expr.debug_visit();
                println!(")");
            }
        }
    }
}