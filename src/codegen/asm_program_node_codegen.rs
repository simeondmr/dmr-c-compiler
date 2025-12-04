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
use std::io::Error;
use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::codegen::asm_codegen_trait::Codegen;
use crate::symbol_table::symbol_table::SymbolTable;

impl Codegen for AsmProgramNode {
    fn codegen(&self, symbol_table: &SymbolTable, output_file: &mut File) -> Result<(), Error> {
        let AsmProgramNode::ProgramAsmDef(functions) = self;
        for function in functions {
            function.codegen(symbol_table, output_file)?;
        }
        Ok(())
    }
}
