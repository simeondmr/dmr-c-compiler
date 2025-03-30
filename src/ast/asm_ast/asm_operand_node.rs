use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_registers_node::Reg;

#[derive(Clone, Debug)]
pub enum OperandAsmNode {
    Imm(i32),
    Register(Reg),
    Pseudo(u32),
    Stack(i32)
}

impl AstAsmDebugPrinter for OperandAsmNode {
    fn debug_visit(&self) {
        match self {
            OperandAsmNode::Imm(value) => print!("Imm({})", value),
            OperandAsmNode::Register(reg) => reg.debug_visit(),
            OperandAsmNode::Pseudo(value) => print!("Pseudo({})", value),
            OperandAsmNode::Stack(value) => print!("Stack({})", value)
        }
    }
}