use crate::ast::asm_ast::asm_ast_visit_trait::AsmReplacingPseudoregisters;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::codegen::stack_alloc_table::StackAllocTable;

impl InstructionAsmNode {
    fn replace_pseudoregister(operand: &mut OperandAsmNode, stack_alloc_table: &mut StackAllocTable) -> OperandAsmNode {
        if let OperandAsmNode::Pseudo(pseudo_value) = operand {
            let src_stack_address = stack_alloc_table.get_or_insert_pseudoregister(*pseudo_value);
            return OperandAsmNode::Stack(src_stack_address)
        }
        operand.clone()
    }
}

impl AsmReplacingPseudoregisters for InstructionAsmNode {
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32 {
        match self {
            InstructionAsmNode::Mov { src, dest } | InstructionAsmNode::Binary { operator: _, src, dest } => {
                let src_replaced = InstructionAsmNode::replace_pseudoregister(src, stack_alloc_table);
                let dest_replaced = InstructionAsmNode::replace_pseudoregister(dest, stack_alloc_table);
                if let InstructionAsmNode::Binary { operator, src: _, dest: _ }= self {
                    *self = InstructionAsmNode::Binary { operator: operator.clone(), src: src_replaced, dest: dest_replaced };
                } else {
                    *self = InstructionAsmNode::Mov { src: src_replaced, dest: dest_replaced };
                }
            },
            InstructionAsmNode::Idiv(dest) => {
                let dest_replaced = InstructionAsmNode::replace_pseudoregister(dest, stack_alloc_table);
                *self = InstructionAsmNode::Idiv(dest_replaced);
            },
            InstructionAsmNode::Unary { operator: unary_operator, operand: OperandAsmNode::Pseudo(value) } => {
                let pseudo_stack_address = stack_alloc_table.get_or_insert_pseudoregister(*value);
                *self = InstructionAsmNode::Unary { operator: unary_operator.clone(), operand: OperandAsmNode::Stack(pseudo_stack_address) };
            },
            InstructionAsmNode::Cmp(operand0, operand1) => {
                let operand0_replaced = InstructionAsmNode::replace_pseudoregister(operand0, stack_alloc_table);
                let operand1_replaced = InstructionAsmNode::replace_pseudoregister(operand1, stack_alloc_table);
                *self = InstructionAsmNode::Cmp(operand0_replaced, operand1_replaced);
            },
            InstructionAsmNode::Set { condition_code, dest } => {
                let dest = InstructionAsmNode::replace_pseudoregister(dest, stack_alloc_table);
                *self = InstructionAsmNode::Set { condition_code: condition_code.clone(), dest };
            },
            InstructionAsmNode::Inc(operand) | InstructionAsmNode::Dec(operand)  => {
                let operand_replaced = InstructionAsmNode::replace_pseudoregister(operand, stack_alloc_table);
                if let InstructionAsmNode::Inc(_) = self {
                    *self = InstructionAsmNode::Inc(operand_replaced);
                } else {
                    *self = InstructionAsmNode::Dec(operand_replaced);
                }
            },
            _ => return 0
        }
        0
    }
}