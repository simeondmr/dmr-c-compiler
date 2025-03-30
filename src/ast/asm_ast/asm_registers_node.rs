use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum Reg {
    AX,
    R10
}

impl AstAsmDebugPrinter for Reg {
    fn debug_visit(&self) {
        match self {
            Reg::AX => print!("AX"),
            Reg::R10 => print!("R10")
        }
    }
}