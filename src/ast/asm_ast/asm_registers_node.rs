use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

#[derive(Clone, Debug)]
pub enum Reg {
    AX,
    CX(RcxReg),
    DX,
    R10,
    R11
}

#[derive(Clone, Debug)]
pub enum RcxReg {
    CL,
    ECX
}

impl AstAsmDebugPrinter for Reg {
    fn debug_visit(&self) {
        match self {
            Reg::AX => print!("AX"),
            Reg::CX(part) => print!("{:?}", part),
            Reg::DX => print!("DX"),
            Reg::R10 => print!("R10"),
            Reg::R11 => print!("R11")
        }
    }
}