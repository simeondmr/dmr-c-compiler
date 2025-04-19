use crate::ast::lang_ast::function_node::FunctionNode::FunctionDef;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky, GenerateTackyInstructions};
use crate::ast::lang_ast::statement_node::StatementNode;
use crate::tacky::tacky_function_node::FunctionTackyNode;

#[derive(Debug)]
pub enum FunctionNode {
    FunctionDef(String, StatementNode)
}

impl GenerateTacky<FunctionTackyNode> for FunctionNode {
    fn to_tacky(&self) -> FunctionTackyNode {
        let FunctionDef(func_name, statement_node) = self;
        let mut tacky_instructions = Vec::new();
        statement_node.to_tacky(&mut tacky_instructions);
        FunctionTackyNode::FunctionDef { func_name: func_name.to_string(), tacky_instructions }
    }
}

impl AstDebugPrinter for FunctionNode {
    fn debug_visit(&self) {
        let FunctionDef(name, stmt) = self;
        println!("Function(");
        println!("name=\"{}\"", name);
        stmt.debug_visit();
        println!(")");
    }
}