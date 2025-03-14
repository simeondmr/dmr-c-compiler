use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::lang_ast::exp_node::ExpNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateAsmAst, GenerateListAsmInstructions};
use crate::ast::lang_ast::statement_node::StatementNode::ReturnStmt;

#[derive(Debug)]
pub enum StatementNode {
    ReturnStmt(ExpNode)
}

impl GenerateListAsmInstructions for StatementNode {
    fn to_asm_ast(&self, asm_instructions: &mut Vec<InstructionAsmNode>) {
        let ReturnStmt(exp_node) = self;
        asm_instructions.push(InstructionAsmNode::Mov(exp_node.to_asm_ast(), OperandAsmNode::Register));
        asm_instructions.push(InstructionAsmNode::Ret);
    }
}

impl AstDebugPrinter for StatementNode {
    fn debug_visit(&self) {
        let ReturnStmt(exp) = self;
        println!("Return(");
        exp.debug_visit();
        println!(")");
    }
}
