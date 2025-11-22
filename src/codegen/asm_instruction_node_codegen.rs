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
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for InstructionAsmNode {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            InstructionAsmNode::Mov { src, dest} => {
                output_file.write_all("\tmovl ".as_bytes())?;
                src.codegen(output_file)?;
                output_file.write_all(", ".as_bytes())?;
                dest.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Unary { operator, operand} => {
                operator.codegen(output_file)?;
                operand.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Binary { operator, src, dest } => {
                operator.codegen(output_file)?;
                src.codegen(output_file)?;
                output_file.write_all(", ".as_bytes())?;
                dest.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)

            },
            InstructionAsmNode::Idiv(operator) => {
                output_file.write_all("\tidivl ".as_bytes())?;
                operator.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Cdq => Ok(output_file.write_all("\tcdq\n".as_bytes())?),
            InstructionAsmNode::AllocateStack(stack_offet) => Ok(output_file.write_all(format!("\tsubq ${}, %rsp\n", stack_offet.abs()).as_bytes())?),
            InstructionAsmNode::DeallocateStack(size) => Ok(output_file.write_all(format!("\taddq ${}, %rsp\n", size).as_bytes())?),
            InstructionAsmNode::Cmp(operand0, operand1) => {
                output_file.write_all("\tcmpl ".as_bytes())?;
                operand0.codegen(output_file)?;
                output_file.write_all(", ".as_bytes())?;
                operand1.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Jmp(jmp_label_target) => Ok(output_file.write_all(format!("\tjmp .l{}\n", jmp_label_target).as_bytes())?),
            InstructionAsmNode::JmpCC { condition_code, jmp_label_target } => Ok(output_file.write_all(format!("\tj{} .l{}\n", condition_code.code(), jmp_label_target).as_bytes())?),
            InstructionAsmNode::Set { condition_code, dest } => {
                /*
                    TODO: at the moment the set destination operand is always a memory operand when registers allocation is done set must use only the first byte of every registers.
                          For example if registers allocator for set decided to use 'rax' register, set must use 'al'
                */
                output_file.write_all(format!("\tset{} ", condition_code.code()).as_bytes())?;
                dest.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Inc(operand) => {
                output_file.write_all("\tincl ".as_bytes())?;
                operand.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Dec(operand) => {
                output_file.write_all("\tdecl ".as_bytes())?;
                operand.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Push(operand) => {
                output_file.write_all("\tpushq ".as_bytes())?;
                operand.codegen(output_file)?;
                Ok(output_file.write_all("\n".as_bytes())?)
            },
            InstructionAsmNode::Call { func_name, has_body} => Ok(output_file.write_all(format!("\tcall {func_name}{}\n", if !has_body { "@PLT" } else { "" }).as_bytes())?),
            InstructionAsmNode::LinuxExitSyscall => Ok(output_file.write_all("\tmovq $60, %rax\n\tsyscall\n".as_bytes())?),
            InstructionAsmNode::Label(index) => Ok(output_file.write_all(format!(".l{index}:\n").as_bytes())?),
            InstructionAsmNode::Ret => {
                output_file.write_all("\tmovq %rbp, %rsp\n".as_bytes())?;
                output_file.write_all("\tpopq %rbp\n".as_bytes())?;
                Ok(output_file.write_all("\tret\n".as_bytes())?)
            }
        }
    }
}