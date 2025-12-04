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

use std::path::Path;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky};
use crate::codegen::codegen_core::CodegenCore;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Lexer;
use crate::parser::program_parse::{GrammarProductionParsing, ProgramParse};
use crate::semantic_analisys;
use crate::symbol_table::symbol_table::SymbolTable;
use crate::tacky::tacky_visit_trait::GenerateAsm;

pub fn compiler_driver(args: Vec<String>) -> Result<(), CompilerErrors> {
    println!("Compiler version: {}", env!("CARGO_PKG_VERSION"));
    if args.len() != 4 {
        eprintln!("Usage: {} <input_file.c> <output_file.asm> <debug_en: -deb_en|-deb_dis>", args.get(0).unwrap());
        return Err(CompilerErrors::WrongCompilerParams);
    }
    let debug = args[3] == "-deb_en";
    let program = ProgramParse::new();
    let mut lexer = Lexer::new(&Path::new(args.get(1).unwrap()));
    let mut ast = program.parse(&mut lexer)?;
    ast_debug("AST visit after parsing:", &ast, debug);
    let mut symbol_table = SymbolTable::new();
    semantic_analisys::semantic_analisys_core::semantic_analisys(&mut ast, &mut symbol_table)?;
    ast_debug("AST visit after semantic analisys:", &ast, debug);
    let tacky = ast.to_tacky();
    ast_debug("Tacky debug:", &tacky, debug);
    let mut asm_ast = tacky.to_asm();
    ast_debug("Asm debug before code fixing:", &asm_ast, debug);
    let codegen_core = CodegenCore::new(&Path::new(args.get(2).unwrap()));
    codegen_core.codegen(&symbol_table, &mut asm_ast).map_err(CompilerErrors::IO)?;
    ast_debug("Asm debug after code fixing", &asm_ast, debug);
    println!("Compilation done");
    Ok(())
}

pub fn ast_debug<T: AstDebugPrinter> (message: &str, ast: &T, debug_en: bool) {
    if debug_en {
        println!("------------------------");
        println!("{message}");
        ast.debug_visit();
        println!("------------------------");
    }
}