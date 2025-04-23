use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;

pub enum AsmProgramNode {
    ProgramAsmDef(FunctionAsmNode)
}

impl AstAsmDebugPrinter for AsmProgramNode {
    fn debug_visit(&self) {
        let AsmProgramNode::ProgramAsmDef(function) = self;
        println!("Program(");
        function.debug_visit();
        println!(")");
    }
}