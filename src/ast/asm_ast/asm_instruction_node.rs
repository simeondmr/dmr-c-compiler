use crate::ast::asm_ast::asm_ast_visit_trait::{AsmReplacingPseudoregisters, AstAsmDebugPrinter};
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperatorNode;
use crate::codegen::stack_alloc_table::StackAllocTable;

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

impl InstructionAsmNode {
    fn replace_operand(operand: &mut OperandAsmNode, stack_alloc_table: &mut StackAllocTable) -> (OperandAsmNode, i32) {
        if let OperandAsmNode::Pseudo(pseudo_value) = operand {
            let src_stack_address = stack_alloc_table.get_or_insert_pseudoregister(*pseudo_value);
            return (OperandAsmNode::Stack(src_stack_address), src_stack_address)
        }

        (operand.clone(), 0)
    }
}

impl AsmReplacingPseudoregisters for InstructionAsmNode {
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32 {
        match self {
            InstructionAsmNode::Mov { src, dest } | InstructionAsmNode::Binary { operator: _, src, dest } => {
                let src_replaced = InstructionAsmNode::replace_operand(src, stack_alloc_table);
                let dest_replaced = InstructionAsmNode::replace_operand(dest, stack_alloc_table);
                if let InstructionAsmNode::Binary { operator, src: _, dest: _ }= self {
                    *self = InstructionAsmNode::Binary { operator: operator.clone(), src: src_replaced.0, dest: dest_replaced.0 };
                } else {
                    *self = InstructionAsmNode::Mov { src: src_replaced.0, dest: dest_replaced.0 };
                }
                src_replaced.1.min(dest_replaced.1)
            },
            InstructionAsmNode::Idiv(OperandAsmNode::Pseudo(value)) => {
                let pseudo_stack_address = stack_alloc_table.get_or_insert_pseudoregister(*value);
                *self = InstructionAsmNode::Idiv( OperandAsmNode::Stack(pseudo_stack_address));
                pseudo_stack_address
            },
            InstructionAsmNode::Unary { operator: unary_operator, operand: OperandAsmNode::Pseudo(value) } => {
                let pseudo_stack_address = stack_alloc_table.get_or_insert_pseudoregister(*value);
                *self = InstructionAsmNode::Unary { operator: unary_operator.clone(), operand: OperandAsmNode::Stack(pseudo_stack_address) };
                pseudo_stack_address
            },
            _ => 0
        }
    }
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