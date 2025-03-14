use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::lang_ast::exp_node::ExpNode::Constant;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateAsmAst};

#[derive(Debug)]
pub enum ExpNode {
    Constant(i32)
}

impl GenerateAsmAst<OperandAsmNode> for ExpNode {
    fn to_asm_ast(&self) -> OperandAsmNode {
        let Constant(value) = self;
        OperandAsmNode::Imm(*value)
    }
}

impl AstDebugPrinter for ExpNode {
    fn debug_visit(&self) {
        let Constant(value) = self;
        println!("Constant: {}", value);
    }
}