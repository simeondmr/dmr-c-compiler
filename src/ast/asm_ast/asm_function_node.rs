use std::collections::VecDeque;
use crate::ast::asm_ast::asm_ast_visit_trait::{AsmReplacingPseudoregisters, AstAsmDebugPrinter, FixingInstruction};
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_registers_node::Reg;
use crate::codegen::stack_alloc_table::StackAllocTable;

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

impl AsmReplacingPseudoregisters for FunctionAsmNode {
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32 {
        let FunctionAsmNode::FunctionAsmDef { func_name: _, ref mut asm_instructions } = self;
        let mut current_stack_offset = 0;
        asm_instructions.iter_mut().for_each(|instruction| {
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
        let FunctionAsmNode::FunctionAsmDef { func_name: _, ref mut asm_instructions } = self;
        asm_instructions.insert(0, InstructionAsmNode::AllocateStack(stack_offset));
        let mut instruction_index= 0;
        while instruction_index < asm_instructions.len() {
            let current_instruction = asm_instructions.get(instruction_index).cloned();
            match current_instruction {
                None => { },
                Some(InstructionAsmNode::Mov { src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Stack(dest_offset) })=> {
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Register(Reg::R10) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Mov { src: OperandAsmNode::Register(Reg::R10), dest: OperandAsmNode::Stack(dest_offset) });
                    instruction_index += 2;
                },
                Some(InstructionAsmNode::Idiv(OperandAsmNode::Imm(value))) => {
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Imm(value), dest: OperandAsmNode::Register(Reg::R10) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Idiv(OperandAsmNode::Register(Reg::R10)));
                    instruction_index += 2;
                },
                Some(InstructionAsmNode::Binary { operator: AsmBinaryOperatorNode::Multiply, src, dest: OperandAsmNode::Stack(dest_offset) }) => {
                    //@Note that imul instruction can't use memory address as its destination so a fix is needed
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(dest_offset), dest: OperandAsmNode::Register(Reg::R11) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Binary { operator: AsmBinaryOperatorNode::Multiply, src, dest: OperandAsmNode::Register(Reg::R11) });
                    asm_instructions.insert(instruction_index + 2, InstructionAsmNode::Mov { src: OperandAsmNode::Register(Reg::R11), dest: OperandAsmNode::Stack(dest_offset) });
                    instruction_index += 3;
                },
                Some(InstructionAsmNode::Binary { operator, src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Stack(dest_offset) }) => {
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Register(Reg::R10) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Binary { operator, src: OperandAsmNode::Register(Reg::R10), dest: OperandAsmNode::Stack(dest_offset) });
                    instruction_index += 2;
                },
                _ => instruction_index += 1
            }
        }
    }
}