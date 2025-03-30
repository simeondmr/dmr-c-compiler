use crate::ast::asm_ast::asm_registers_node::Reg;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for Reg {
    fn codegen(&self) {
        match self {
            Reg::AX => print!("%eax"),
            Reg::R10 => print!("%r10d")
        }
    }
}