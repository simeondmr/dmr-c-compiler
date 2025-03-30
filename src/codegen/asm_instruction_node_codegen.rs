use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for InstructionAsmNode {
    fn codegen(&self) {
        match self {
            InstructionAsmNode::Mov(src_operand, dest_operand) => {
                print!("movl ");
                src_operand.codegen();
                print!(", ");
                dest_operand.codegen();
                println!();
            },
            InstructionAsmNode::Unary(operator, operand) => {
                operator.codegen();
                operand.codegen();
                println!()
            },
            InstructionAsmNode::AllocateStack(stack_offet) => {
                println!("subq {}, %rsp", *stack_offet);
            },
            InstructionAsmNode::Ret => {
                println!("movq %rbp, %rsp");
                println!("popq %rbp");
                println!("ret");
            }
        }
    }
}