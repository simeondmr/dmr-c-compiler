// dmr C compiler
// Copyright (C) 2025  Simeon Tornabene
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, see <https://www.gnu.org/licenses/>.

use crate::ast::asm_ast::asm_ast_visit_trait::FixingInstruction;
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_registers_node::{RcxReg, Reg};

impl FixingInstruction for FunctionAsmNode {
    fn fixing_instructions(&mut self) {
        let FunctionAsmNode::FunctionAsmDef { func_name: _, stack_alloc_size, ref mut asm_instructions } = self;
        if *stack_alloc_size != 0 {
            //Note this the stack alloc for function prologue with alignment(remember that in System V ABI, the  stack must be 16-byte aligned)
            asm_instructions.insert(0, InstructionAsmNode::AllocateStack(*stack_alloc_size + (*stack_alloc_size % 16)));
        }
        let mut instruction_index= 0;
        while instruction_index < asm_instructions.len() {
            let current_instruction = asm_instructions.get(instruction_index).cloned();
            match current_instruction {
                Some(InstructionAsmNode::Mov { src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Stack(dest_offset) }) => {
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
                    // Note: imul instruction can't use memory address as its destination(right operand)
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(dest_offset), dest: OperandAsmNode::Register(Reg::R11) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Binary { operator: AsmBinaryOperatorNode::Multiply, src, dest: OperandAsmNode::Register(Reg::R11) });
                    asm_instructions.insert(instruction_index + 2, InstructionAsmNode::Mov { src: OperandAsmNode::Register(Reg::R11), dest: OperandAsmNode::Stack(dest_offset) });
                    instruction_index += 3;
                },
                Some(InstructionAsmNode::Binary { operator, src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Stack(dest_offset) }) if matches!(operator, AsmBinaryOperatorNode::BitwiseLeftShift | AsmBinaryOperatorNode::BitwiseRightShift) => {
                    // Note: instructions sal, shl, sar and shr if the first parameter is not an immediate value, it must be copied before into CL register(lower byte of RCX register), and cl must be used as first bitwise shift instruction argument
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Register(Reg::CX(RcxReg::ECX)) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Binary { operator, src: OperandAsmNode::Register(Reg::CX(RcxReg::CL)), dest: OperandAsmNode::Stack(dest_offset) });
                    instruction_index += 2;
                },
                Some(InstructionAsmNode::Binary { operator, src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Stack(dest_offset) }) => {
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(src_offset), dest: OperandAsmNode::Register(Reg::R10) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Binary { operator, src: OperandAsmNode::Register(Reg::R10), dest: OperandAsmNode::Stack(dest_offset) });
                    instruction_index += 2;
                },
                Some(InstructionAsmNode::Cmp(OperandAsmNode::Stack(offset_operand0), OperandAsmNode::Stack(offset_operand1))) => {
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(offset_operand0), dest: OperandAsmNode::Register(Reg::R10) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Cmp(OperandAsmNode::Register(Reg::R10), OperandAsmNode::Stack(offset_operand1)));
                    instruction_index += 2;
                },
                Some(InstructionAsmNode::Cmp(operand0, OperandAsmNode::Imm(value))) => {
                    // Note: cmp instruction cannot have an immediate value in the right operand
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Imm(value), dest: OperandAsmNode::Register(Reg::R11) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Cmp(operand0, OperandAsmNode::Register(Reg::R11)));
                    instruction_index += 2;
                },
                Some(InstructionAsmNode::Push(OperandAsmNode::Stack(offset))) => {
                    asm_instructions[instruction_index] = InstructionAsmNode::Mov { src: OperandAsmNode::Stack(offset), dest: OperandAsmNode::Register(Reg::R10) };
                    asm_instructions.insert(instruction_index + 1, InstructionAsmNode::Push(OperandAsmNode::Register(Reg::R10)));
                    instruction_index += 2;
                },
                _ => instruction_index += 1
            }
        }
    }
}