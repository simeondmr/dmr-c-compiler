use std::collections::VecDeque;
use crate::ast::asm_ast::asm_ast_visit_trait::{AsmReplacingPseudoregisters, AstAsmDebugPrinter, FixingInstruction};
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_registers_node::Reg;
use crate::codegen::stack_alloc_table::StackAllocTable;

/// This enum provides a Node for defining functions node
pub enum FunctionAsmNode {

    /// Function node
    /// # Fields
    /// * name - defined as String
    /// * instructions - defined as VecDeque<InstructionAsmNode>
    FunctionAsmDef(String, VecDeque<InstructionAsmNode>)
}

impl AstAsmDebugPrinter for FunctionAsmNode {
    fn debug_visit(&self) {
        let FunctionAsmNode::FunctionAsmDef(name, ref instructions) = self;
        println!("Function(\nname = {}", name);
        instructions.iter().for_each(|instruction| instruction.debug_visit());
        println!(")");
    }
}

impl AsmReplacingPseudoregisters for FunctionAsmNode {
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32 {
        let FunctionAsmNode::FunctionAsmDef(_, ref mut instructions) = self;
        let mut current_stack_offset = 0;
        instructions.iter_mut().for_each(|instruction| {
            let instruction_param_stack_offset = instruction.replacing_pseudoregisters(stack_alloc_table);
            if current_stack_offset > instruction_param_stack_offset {
                current_stack_offset = instruction_param_stack_offset;
            }
        });
        current_stack_offset
    }
}

impl FixingInstruction for FunctionAsmNode {
    fn fixing_instructions(&mut self, stack_offset: i32) {
        let FunctionAsmNode::FunctionAsmDef(_, ref mut instructions) = self;
        instructions.insert(0, InstructionAsmNode::AllocateStack(stack_offset));
        let mut instruction_index= 0;
        while instruction_index < instructions.len() {
            if let Some(InstructionAsmNode::Mov(OperandAsmNode::Stack(src), OperandAsmNode::Stack(dest))) = instructions.get(instruction_index).cloned() {
                instructions[instruction_index] = InstructionAsmNode::Mov(OperandAsmNode::Stack(src), OperandAsmNode::Register(Reg::R10));
                instructions.insert(instruction_index + 1, InstructionAsmNode::Mov(OperandAsmNode::Register(Reg::R10), OperandAsmNode::Stack(dest)));
                instruction_index += 2;
            } else {
                instruction_index += 1;
            }
        }
    }
}