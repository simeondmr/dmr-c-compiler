use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;

pub enum OperandAsmNode {
    Imm(i32),
    Register
}

impl AstAsmDebugPrinter for OperandAsmNode {
    fn debug_visit(&mut self) {
        if let OperandAsmNode::Imm(value) = self {
            print!("${}", value);
        } else if let OperandAsmNode::Register = self {
            print!("%eax");
        }
    }
}