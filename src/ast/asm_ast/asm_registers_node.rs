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

use crate::ast::lang_ast::lang_ast_visit_trait::AstDebugPrinter;

#[derive(Clone, Debug)]
pub enum Reg {
    AX(RaxReg),
    CX(RcxReg),
    DX,
    R8,
    R9,
    R10,
    R11,
    DI,
    SI,
}

#[derive(Clone, Debug)]
pub enum RcxReg {
    CL,
    ECX
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum RaxReg {
    XL,
    EAX,
    RAX
}

impl AstDebugPrinter for Reg {
    fn debug_visit(&self) {
        match self {
            Reg::AX(rax_reg) => print!("{:?}", rax_reg),
            Reg::CX(part) => print!("{:?}", part),
            Reg::DX => print!("DX"),
            Reg::R8 => print!("R8"),
            Reg::R9 => print!("R9"),
            Reg::R10 => print!("R10"),
            Reg::R11 => print!("R11"),
            Reg::DI => print!("DI"),
            Reg::SI => print!("SI")
        }
    }
}