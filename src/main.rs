use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::lang_ast::lang_ast_visit_trait::GenerateAsmAst;
use crate::parser::program::{GrammarProductionParsing, Program};
use crate::errors::errors::CompilerErrors;

mod lexer;
mod parser;
mod errors;
mod ast;

fn main() -> Result<(), CompilerErrors>  {
    let program = Program::new();
    let ast = program.parse()?;

    let mut asm_ast = ast.to_asm_ast();

    asm_ast.debug_visit();

    Ok(())
}
