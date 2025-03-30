use std::collections::VecDeque;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;

pub trait TackyVisitDebug {
    fn visit_debug(&self);
}

pub trait GenerateAsm<T> {
    fn to_asm(&self) -> T;
}

pub trait GenerateAsmInstruction<T> {
    fn to_asm(&self, asm_instructions: &mut VecDeque<InstructionAsmNode>) -> T;
}