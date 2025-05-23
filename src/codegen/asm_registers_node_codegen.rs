use std::fs::File;
use std::io::{Error, Write};
use crate::ast::asm_ast::asm_registers_node::Reg;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for Reg {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error> {
        match self {
            Reg::AX => Ok(output_file.write_all("%eax".as_bytes())?),
            Reg::CX(part) => Ok(output_file.write_all(format!("%{:?}", part).as_bytes())?),
            Reg::DX => Ok(output_file.write_all("%edx".as_bytes())?),
            Reg::R10 => Ok(output_file.write_all("%r10d".as_bytes())?),
            Reg::R11 => Ok(output_file.write_all("%r11d".as_bytes())?)
        }
    }
}