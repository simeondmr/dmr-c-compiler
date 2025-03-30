use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for OperandAsmNode {
    fn codegen(&self) {
        match self {
            OperandAsmNode::Imm(value) => print!("${}", value),
            OperandAsmNode::Register(reg) => reg.codegen(),
            OperandAsmNode::Pseudo(value) => print!("t{}", value),
            OperandAsmNode::Stack(value) => print!("{}(%rsp)", value)
        }
    }
}