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
use std::path::Path;
use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky};
use crate::codegen::codegen_core::CodegenCore;
use crate::parser::program_parse::{GrammarProductionParsing, ProgramParse};
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Lexer;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

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

fn main() -> Result<(), CompilerErrors>  {
    let args: Vec<String> = env::args().collect();
    println!("Compiler version: {}", env!("CARGO_PKG_VERSION"));
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file.c> <output_file.asm>", args.get(0).unwrap());
        return Err(CompilerErrors::WrongParams);
    }
    let program = ProgramParse::new();
    let mut lexer = Lexer::new(&Path::new(args.get(1).unwrap()));
    let mut ast = program.parse(&mut lexer)?;
    println!("------------------------");
    println!("AST visit after parsing:");
    ast.debug_visit();
    println!("------------------------");
    semantic_analisys::semantic_analisys_core::semantic_analisys(&mut ast)?;
    println!("AST visit after semantic analisys:");
    ast.debug_visit();
    println!("------------------------");
    let tacky = ast.to_tacky();
    println!("Tacky debug:");
    tacky.visit_debug();
    println!("------------------------");
    let mut asm_ast = tacky.to_asm();
    println!("Asm debug before code fixing:");
    asm_ast.debug_visit();
    println!("------------------------");
    let codegen_core = CodegenCore::new(&Path::new(args.get(2).unwrap()));
    codegen_core.codegen(&mut asm_ast).map_err(CompilerErrors::IO)?;
    println!("Asm debug after code fixing");
    asm_ast.debug_visit();
    println!("------------------------");
    Ok(())
}