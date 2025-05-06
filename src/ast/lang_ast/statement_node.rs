use crate::ast::lang_ast::expr_node::ExprNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;

#[derive(Debug)]
pub enum StatementNode {
    ReturnStmt(ExprNode),
    Expr(ExprNode),
    EmptyStmt,
}

impl GenerateTackyInstructions<()> for StatementNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        match self {
            StatementNode::ReturnStmt(expr) => {
                let expr_tacky = expr.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::Return(expr_tacky))
            },
            StatementNode::Expr(expr) =>  {
                expr.to_tacky(tacky_instructions);
            },
            StatementNode::EmptyStmt => {
                // Note: nothing to do
            }
        }
    }
}

impl AstDebugPrinter for StatementNode {
    fn debug_visit(&self) {
        match self {
            StatementNode::ReturnStmt(expr) => {
                println!("Return(");
                expr.debug_visit();
                println!(")");
            }
            StatementNode::Expr(expr) => expr.debug_visit(),
            StatementNode::EmptyStmt => println!("EmptyStmt")
        }
    }
}
