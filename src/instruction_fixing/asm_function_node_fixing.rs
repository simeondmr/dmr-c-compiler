use crate::ast::asm_ast::asm_ast_visit_trait::FixingInstruction;
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_registers_node::{RcxReg, Reg};

impl FixingInstruction for FunctionAsmNode {
    fn fixing_instructions(&mut self, stack_offset: i32) {
        let FunctionAsmNode::FunctionAsmDef { func_name: _, ref mut asm_instructions } = self;
        asm_instructions.insert(0, InstructionAsmNode::AllocateStack(stack_offset));
        let mut instruction_index= 0;
        while instruction_index < asm_instructions.len() {
            let current_instruction = asm_instructions.get(instruction_index).cloned();
            match current_instruction {
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
                Some(InstructionAsmNode::Binary { operator, src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Stack(dest_offset) }) if matches!(operator, AsmBinaryOperatorNode::BitwiseLeftShift | AsmBinaryOperatorNode::BitwiseRightShift) => {
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Register(Reg::CX(RcxReg::ECX)) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Binary { operator, src: OperandAsmNode::Register(Reg::CX(RcxReg::CL)), dest: OperandAsmNode::Stack(dest_offset) });
                    instruction_index += 2;
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