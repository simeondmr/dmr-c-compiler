use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;

pub enum ProgramAsmNode {
    ProgramAsmDef(FunctionAsmNode)
}

impl AstAsmDebugPrinter for ProgramAsmNode {
    fn debug_visit(&mut self) {
        let ProgramAsmNode::ProgramAsmDef(function) = self;
        function.debug_visit();
    }
}