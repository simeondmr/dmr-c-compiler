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