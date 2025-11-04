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

use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum Reg {
    AX,
    CX(RcxReg),
    DX,
    R10,
    R11
}

#[derive(Clone, Debug)]
pub enum RcxReg {
    CL,
    ECX
}

impl AstAsmDebugPrinter for Reg {
    fn debug_visit(&self) {
        match self {
            Reg::AX => print!("AX"),
            Reg::CX(part) => print!("{:?}", part),
            Reg::DX => print!("DX"),
            Reg::R10 => print!("R10"),
            Reg::R11 => print!("R11")
        }
    }
}