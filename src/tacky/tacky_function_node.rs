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
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, GenerateAsmInstruction, TackyVisitDebug};

pub enum FunctionTackyNode {
    FunctionDef {
        func_name: String,
        tacky_instructions: Vec<InstructionTackyNode>
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

impl TackyVisitDebug for FunctionTackyNode{
    fn visit_debug(&self) {
        let FunctionTackyNode::FunctionDef { func_name, tacky_instructions} = self;
        println!("Name: {}", func_name);
        tacky_instructions.iter().for_each(|instruction| instruction.visit_debug());
    }
}