use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::ast::lang_ast::function_node::FunctionNode::FunctionDef;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateAsmAst, GenerateListAsmInstructions};
use crate::ast::lang_ast::statement_node::StatementNode;

#[derive(Debug)]
pub enum FunctionNode {
    FunctionDef(String, StatementNode)
}

impl GenerateAsmAst<FunctionAsmNode> for FunctionNode {
    fn to_asm_ast(&self) -> FunctionAsmNode {
        let FunctionDef(func_name, stmt_node) = self;
        let mut asm_instructions = Vec::new();
        stmt_node.to_asm_ast(&mut asm_instructions);
        FunctionAsmNode::FunctionAsmDef(func_name.to_string(), asm_instructions)
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