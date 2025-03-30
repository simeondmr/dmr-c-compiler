use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::lang_ast::lang_ast_visit_trait::GenerateTacky;
use crate::codegen::codegen_core::CodegenCore;
use crate::parser::program::{GrammarProductionParsing, Program};
use crate::errors::errors::CompilerErrors;
use crate::tacky::tacky_visit_trait::GenerateAsm;

mod lexer;
mod parser;
mod errors;
mod ast;
mod tacky;
mod codegen;

fn main() -> Result<(), CompilerErrors>  {
    let program = Program::new();
    let ast = program.parse()?;

    let tacky = ast.to_tacky();
    let mut asm_ast = tacky.to_asm();
    let codegen_core = CodegenCore::new();
    codegen_core.codegen(&mut asm_ast);
    println!("------------------------");
    asm_ast.debug_visit();
    Ok(())
}