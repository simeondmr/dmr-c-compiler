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

pub mod asm_function_node_codegen;
pub mod asm_instruction_node_codegen;
pub mod asm_operand_node_codegen;
pub mod asm_program_node_codegen;
pub mod asm_registers_node_codegen;
pub mod asm_unary_operator_node_codegen;
pub mod asm_codegen_trait;
pub mod codegen_core;
pub mod stack_alloc_table;
pub mod asm_binary_operator_node_codegen;