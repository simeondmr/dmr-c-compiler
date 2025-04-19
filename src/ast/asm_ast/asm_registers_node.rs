use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum Reg {
    AX,
    DX,
    R10,
    R11
}

impl AstAsmDebugPrinter for Reg {
    fn debug_visit(&self) {
        match self {
            Reg::AX => print!("AX"),
            Reg::DX => print!("DX"),
            Reg::R10 => print!("R10"),
            Reg::R11 => print!("R11")
        }
    }
}