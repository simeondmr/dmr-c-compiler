use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;

pub enum InstructionAsmNode {
    Mov(OperandAsmNode, OperandAsmNode),
    Ret
}

impl AstAsmDebugPrinter for InstructionAsmNode {
    fn debug_visit(&mut self) {
        if let InstructionAsmNode::Mov(src_operand, dest_operand) = self {
            print!("mov ");
            src_operand.debug_visit();
            print!(", ");
            dest_operand.debug_visit();
            println!();
        } else if let InstructionAsmNode::Ret = self {
            println!("ret");
        }
    }
}