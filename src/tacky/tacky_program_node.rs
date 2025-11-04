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

use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::tacky::tacky_function_node::FunctionTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

pub enum ProgramTackyNode {
    ProgramDef(FunctionTackyNode)
}

impl TackyVisitDebug for ProgramTackyNode {
    fn visit_debug(&self) {
        let ProgramTackyNode::ProgramDef(function) = self;
        function.visit_debug();
    }
}

impl GenerateAsm<AsmProgramNode> for ProgramTackyNode {
    fn to_asm(&self) -> AsmProgramNode {
        let ProgramTackyNode::ProgramDef(function) = self;
        AsmProgramNode::ProgramAsmDef(function.to_asm())
    }
}