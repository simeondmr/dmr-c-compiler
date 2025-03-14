use crate::ast::asm_ast::program_asm_node::ProgramAsmNode;
use crate::ast::lang_ast::function_node::FunctionNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateAsmAst};
use crate::ast::lang_ast::program_node::ProgramNode::ProgramDef;

#[derive(Debug)]
pub enum ProgramNode {
    ProgramDef(FunctionNode)
}

impl GenerateAsmAst<ProgramAsmNode> for ProgramNode {
    fn to_asm_ast(&self) -> ProgramAsmNode {
        let ProgramDef(func_node) = self;
        ProgramAsmNode::ProgramAsmDef(func_node.to_asm_ast())
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