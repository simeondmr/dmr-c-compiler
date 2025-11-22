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

use std::collections::VecDeque;
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_registers_node::{RcxReg, Reg};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, GenerateAsmInstruction, TackyVisitDebug};

pub enum FunctionTackyNode {
    FunctionDef {
        func_name: String,
        params: Vec<String>,
        tacky_instructions: Vec<InstructionTackyNode>,
    }
}

impl GenerateAsm<FunctionAsmNode> for FunctionTackyNode {
    fn to_asm(&self) -> FunctionAsmNode {
        let FunctionTackyNode::FunctionDef { func_name, params, tacky_instructions } = self;
        let mut asm_instructions = VecDeque::new();
        let arg_registers = [Reg::DI, Reg::SI, Reg::DX, Reg::CX(RcxReg::ECX), Reg::R8, Reg::R9];
        let registers_len = arg_registers.len();
        let param_stack_len = (params.len().saturating_sub(arg_registers.len())) as u64;
        for i  in  0..params.len() - param_stack_len as usize {
            asm_instructions.push_back(InstructionAsmNode::Mov { src: OperandAsmNode::Register(arg_registers[i].clone()), dest: OperandAsmNode::Pseudo((i + 1) as u32) });
        }
        let stack_param_offset = 16;
        /*
            Note: very important!
            As you can see stack_param_offset start from byte offset 16 in order to start point to 16(%rbp)
            So the first stack params starts at 16(%rbp), the second at 24(%rbp) and so on...
            Remember that in 8(%rbp) there is the functions return address
         */
        for i in 0.. param_stack_len as usize {
            asm_instructions.push_back(InstructionAsmNode::Mov { src: OperandAsmNode::Stack(stack_param_offset + (i as i32 * 8)), dest: OperandAsmNode::Pseudo((i + registers_len + 1) as u32) });
        }
        tacky_instructions.into_iter().for_each(|instruction| instruction.to_asm(&mut asm_instructions));
        FunctionAsmNode::FunctionAsmDef { func_name: func_name.clone(), stack_alloc_size: 0, asm_instructions }
    }
}

impl TackyVisitDebug for FunctionTackyNode{
    fn visit_debug(&self) {
        let FunctionTackyNode::FunctionDef { func_name, params: _, tacky_instructions } = self;
        println!("Name: {}", func_name);
        tacky_instructions.iter().for_each(|instruction| instruction.visit_debug());
    }
}