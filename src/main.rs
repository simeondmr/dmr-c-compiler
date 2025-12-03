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

use std::env;
use crate::compiler_driver::compiler_driver::compiler_driver;
use crate::errors::errors::CompilerErrors;

mod lexer;
mod parser;
mod errors;
mod ast;
mod tacky;
mod codegen;
mod instruction_fixing;
mod replacing_pseudoregisters;
mod semantic_analisys;
mod symbol_table;
mod compiler_driver;

fn main() -> Result<(), CompilerErrors>  {
    compiler_driver(env::args().collect())
}