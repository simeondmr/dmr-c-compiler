use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;

pub enum FunctionAsmNode {
    FunctionAsmDef(String, Vec<InstructionAsmNode>)
}

impl AstAsmDebugPrinter for FunctionAsmNode {
    fn debug_visit(&mut self) {
        let FunctionAsmNode::FunctionAsmDef(name, ref mut instructions) = self;
        println!(".globl {}", name);
        println!("{}:", name);
        instructions.iter_mut().for_each(|instruction| instruction.debug_visit());
    }
}