use crate::ast::lang_ast::exp_node::ExpNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::ast::lang_ast::statement_node::StatementNode::ReturnStmt;
use crate::tacky::tacky_instruction_node::InstructionTackyNode;

#[derive(Debug)]
pub enum StatementNode {
    ReturnStmt(ExpNode)
}

impl GenerateTackyInstructions<()> for StatementNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        let ReturnStmt(exp_node) = self;
        let value = exp_node.to_tacky(tacky_instructions);
        tacky_instructions.push(InstructionTackyNode::Return(value))
    }
}

impl AstDebugPrinter for StatementNode {
    fn debug_visit(&self) {
        let ReturnStmt(exp) = self;
        println!("Return(");
        exp.debug_visit();
        println!(")");
    }
}
