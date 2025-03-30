use crate::ast::asm_ast::asm_ast_visit_trait::{AsmReplacingPseudoregisters, AstAsmDebugPrinter};
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperator;
use crate::codegen::stack_alloc_table::StackAllocTable;

#[derive(Clone)]
pub enum InstructionAsmNode {
    Mov(OperandAsmNode, OperandAsmNode),
    Unary(AsmUnaryOperator, OperandAsmNode),
    AllocateStack(i32),
    Ret
}

impl AstAsmDebugPrinter for InstructionAsmNode {
    fn debug_visit(&self) {
        match self {
            InstructionAsmNode::Mov(src_operand, dest_operand) => {
                print!("Mov ");
                src_operand.debug_visit();
                print!(", ");
                dest_operand.debug_visit();
                println!();
            },
            InstructionAsmNode::Unary(operator, operand) => {
                operator.debug_visit();
                operand.debug_visit();
                println!()
            },
            InstructionAsmNode::AllocateStack(stack_offet) => {
                println!("AllocateStack {}", *stack_offet);
            },
            InstructionAsmNode::Ret => {
                println!("Ret");
            }
        }
    }
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
            InstructionAsmNode::Mov(src, dest) => {
                let src_replaced = InstructionAsmNode::replace_operand(src, stack_alloc_table);
                let dest_replaced = InstructionAsmNode::replace_operand(dest, stack_alloc_table);
                *self = InstructionAsmNode::Mov(src_replaced.0, dest_replaced.0);
                src_replaced.1.max(dest_replaced.1)
            },
            InstructionAsmNode::Unary(unary_operator, OperandAsmNode::Pseudo(pseudo_value)) => {
                let pseudo_stack_address = stack_alloc_table.get_or_insert_pseudoregister(*pseudo_value);
                *self = InstructionAsmNode::Unary(unary_operator.clone(), OperandAsmNode::Stack(pseudo_stack_address));
                pseudo_stack_address
            },
            _ => 0
        }
    }
}