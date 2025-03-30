use crate::ast::lang_ast::exp_node::ExpNode::{Constant, Unary};
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky, GenerateTackyInstructions};
use crate::ast::lang_ast::unary_operator_node::UnaryOperatorNode;
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_val_node::{TemporaryVar, ValTackyNode};


#[derive(Debug)]
pub enum ExpNode {
    Constant(i32),
    Unary(UnaryOperatorNode, Box<ExpNode>)
}

impl GenerateTackyInstructions<ValTackyNode> for ExpNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> ValTackyNode {
        match self {
            Constant(value) => ValTackyNode::Constant(*value),
            Unary(unary_operator, exp_node) => {
                let source = exp_node.to_tacky(tacky_instructions);
                let destination = ValTackyNode::Var(TemporaryVar::generate());
                tacky_instructions.push(InstructionTackyNode::Unary(unary_operator.to_tacky(), source, destination.clone()));
                destination
            }
        }
    }
}

impl AstDebugPrinter for ExpNode {
    fn debug_visit(&self) {
        match self {
            Constant(value) => println!("Constant({})", value),
            Unary(unary_operator, exp_node) => {
                println!("Unary {:?} (", unary_operator);
                exp_node.debug_visit();
                println!(")");
            }
        }
    }
}