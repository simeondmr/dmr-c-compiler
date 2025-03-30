use std::collections::VecDeque;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, GenerateAsmInstruction, TackyVisitDebug};

pub enum FunctionTackyNode {
    FunctionDef(String, Vec<InstructionTackyNode>)
}

impl TackyVisitDebug for FunctionTackyNode{
    fn visit_debug(&self) {
        let FunctionTackyNode::FunctionDef(name, instructions) = self;
        println!("Name: {}", name);
        instructions.iter().for_each(|instruction| instruction.visit_debug());
    }
}

impl GenerateAsm<FunctionAsmNode> for FunctionTackyNode {
    fn to_asm(&self) -> FunctionAsmNode {
        let FunctionTackyNode::FunctionDef(name, instructions) = self;
        let mut asm_instructions = VecDeque::new();
        instructions.into_iter().for_each(|instruction| instruction.to_asm(&mut asm_instructions));
        FunctionAsmNode::FunctionAsmDef(name.clone(), asm_instructions)
    }
}