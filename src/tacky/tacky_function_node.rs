use std::collections::VecDeque;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, GenerateAsmInstruction, TackyVisitDebug};

pub enum FunctionTackyNode {
    FunctionDef {
        func_name: String,
        tacky_instructions: Vec<InstructionTackyNode>
    }
}

impl TackyVisitDebug for FunctionTackyNode{
    fn visit_debug(&self) {
        let FunctionTackyNode::FunctionDef { func_name, tacky_instructions} = self;
        println!("Name: {}", func_name);
        tacky_instructions.iter().for_each(|instruction| instruction.visit_debug());
    }
}

impl GenerateAsm<FunctionAsmNode> for FunctionTackyNode {
    fn to_asm(&self) -> FunctionAsmNode {
        let FunctionTackyNode::FunctionDef { func_name, tacky_instructions } = self;
        let mut asm_instructions = VecDeque::new();
        tacky_instructions.into_iter().for_each(|instruction| instruction.to_asm(&mut asm_instructions));
        FunctionAsmNode::FunctionAsmDef { func_name: func_name.clone(), asm_instructions }
    }
}