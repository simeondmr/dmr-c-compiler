use crate::ast::lang_ast::function_node::FunctionNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky};
use crate::ast::lang_ast::program_node::ProgramNode::ProgramDef;
use crate::tacky::tacky_program_node::ProgramTackyNode;

#[derive(Debug)]
pub enum ProgramNode {
    ProgramDef(FunctionNode)
}

impl GenerateTacky<ProgramTackyNode> for ProgramNode {
    fn to_tacky(&self) -> ProgramTackyNode {
        let ProgramDef(func_node) = self;
        ProgramTackyNode::ProgramDef(func_node.to_tacky())
    }
}

impl AstDebugPrinter for ProgramNode {
    fn debug_visit(&self) {
        let ProgramDef(function_def) = self;
        println!("Program(");
        function_def.debug_visit();
        println!(")");
    }
}