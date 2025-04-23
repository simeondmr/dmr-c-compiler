use crate::ast::asm_ast::asm_ast_visit_trait::AstAsmDebugPrinter;
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperatorNode;

#[derive(Clone)]
pub enum InstructionAsmNode {
    Mov {
        src: OperandAsmNode,
        dest: OperandAsmNode,
    },
    Unary {
        operator: AsmUnaryOperatorNode,
        operand: OperandAsmNode
    },
    Binary {
        operator: AsmBinaryOperatorNode,
        src: OperandAsmNode,
        dest: OperandAsmNode
    },
    Idiv(OperandAsmNode),
    Cdq,
    AllocateStack(i32),
    Ret
}

impl AstAsmDebugPrinter for InstructionAsmNode {
    fn debug_visit(&self) {
        match self {
            InstructionAsmNode::Mov { src, dest } => {
                print!("Mov ");
                src.debug_visit();
                print!(", ");
                dest.debug_visit();
                println!();
            },
            InstructionAsmNode::Unary { operator, operand } => {
                operator.debug_visit();
                operand.debug_visit();
                println!()
            },
            InstructionAsmNode::Binary { operator, src, dest } => {
                operator.debug_visit();
                src.debug_visit();
                println!(", ");
                dest.debug_visit();
                println!();
            },
            InstructionAsmNode::Idiv(operand) => {
                print!("idiv ");
                operand.debug_visit();
            },
            InstructionAsmNode::Cdq => println!("cdq"),
            InstructionAsmNode::AllocateStack(stack_offet) => println!("AllocateStack {}", *stack_offet),
            InstructionAsmNode::Ret => println!("Ret")

        }
    }
}