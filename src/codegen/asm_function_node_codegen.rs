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
use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for FunctionAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        let FunctionAsmNode::FunctionAsmDef { func_name, stack_alloc_size: _, ref asm_instructions } = self;
        if func_name.eq("main") {
            output_file.write_all(format!(".globl _start\n_start:\n").as_bytes())?;
            output_file.write_all(format!("\tcall main\n").as_bytes())?;
            output_file.write_all(format!("\tmovq %rax, %rdi\n").as_bytes())?;
            output_file.write_all(format!("\tmovq $60, %rax\n").as_bytes())?;
            output_file.write_all(format!("\tsyscall\n").as_bytes())?;
        }
        output_file.write_all(format!(".globl {}\n", func_name).as_bytes())?;
        output_file.write_all(format!("{}:\n", func_name).as_bytes())?;
        output_file.write_all("\tpushq %rbp\n".as_bytes())?;
        output_file.write_all("\tmovq %rsp, %rbp\n".as_bytes())?;
        asm_instructions.iter().try_for_each(|instruction| instruction.codegen(output_file))?;
        Ok(())
    }
}