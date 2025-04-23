use std::collections::VecDeque;
use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;

/// This enum provides a Node for defining functions node
pub enum FunctionAsmNode {
    FunctionAsmDef {
        func_name: String,
        asm_instructions: VecDeque<InstructionAsmNode>
    }
}

impl AstAsmDebugPrinter for FunctionAsmNode {
    fn debug_visit(&self) {
        let FunctionAsmNode::FunctionAsmDef { func_name, ref asm_instructions} = self;
        println!("Function(\nname = {}", func_name);
        asm_instructions.iter().for_each(|instruction| instruction.debug_visit());
        println!(")");
    }
}