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

use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for OperandAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error>{
        match self {
            OperandAsmNode::Imm(value) => Ok(output_file.write_all(format!("${}", value).as_bytes())?),
            OperandAsmNode::Register(reg) => reg.codegen(output_file),
            OperandAsmNode::Pseudo(value) => Ok(output_file.write_all(format!("t{}", value).as_bytes())?),
            OperandAsmNode::Stack(value) => Ok(output_file.write_all(format!("{}(%rsp)", value).as_bytes())?),
            _ => { 
                // Note: this code will never be executed so, for Empty case there is nothing to do
                Ok(())
            }
        }
    }
}