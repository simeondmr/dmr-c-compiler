use std::env;
use std::path::Path;
use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::lang_ast::lang_ast_visit_trait::GenerateTacky;
use crate::codegen::codegen_core::CodegenCore;
use crate::parser::program::{GrammarProductionParsing, Program};
use crate::errors::errors::CompilerErrors;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

mod lexer;
mod parser;
mod errors;
mod ast;
mod tacky;
mod codegen;

fn main() -> Result<(), CompilerErrors>  {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input_file.c> <output_file.asm>", args.get(0).unwrap());
        return Err(CompilerErrors::WrongParams);
    }

    let program = Program::new(&Path::new(args.get(1).unwrap()));
    let ast = program.parse()?;
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