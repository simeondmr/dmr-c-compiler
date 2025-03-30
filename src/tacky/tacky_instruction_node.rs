use std::collections::VecDeque;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_registers_node::Reg;
use crate::tacky::tacky_unary_operator_node::UnaryOperatorTackyNode;
use crate::tacky::tacky_val_node::ValTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, GenerateAsmInstruction, TackyVisitDebug};

pub enum InstructionTackyNode {
    Return(ValTackyNode),
    Unary(UnaryOperatorTackyNode, ValTackyNode, ValTackyNode)
}

impl TackyVisitDebug for InstructionTackyNode {
    fn visit_debug(&self) {
        match self {
            InstructionTackyNode::Return(val) => {
                println!("Return(");
                val.visit_debug();
                println!(")");
            },
            InstructionTackyNode::Unary(unary_operator, src, dest) => {
                println!("UnaryOperator(");
                unary_operator.visit_debug();
                print!("src: ");
                src.visit_debug();
                print!("dest: ");
                dest.visit_debug();
                println!(")");
            }
        }
    }
}

impl GenerateAsmInstruction<()> for InstructionTackyNode {
    fn to_asm(&self, asm_instructions: &mut VecDeque<InstructionAsmNode>) -> () {
        match self {
            InstructionTackyNode::Return(val) =>  {
                let src_operand = val.to_asm();
                asm_instructions.push_back(InstructionAsmNode::Mov(src_operand, OperandAsmNode::Register(Reg::AX)));
                asm_instructions.push_back(InstructionAsmNode::Ret);
            },
            InstructionTackyNode::Unary(unary_operand, src, dest) => {
                let asm_dest_val = dest.to_asm();
                asm_instructions.push_back(InstructionAsmNode::Mov(src.to_asm(), asm_dest_val.clone()));
                asm_instructions.push_back(InstructionAsmNode::Unary(unary_operand.to_asm(), asm_dest_val));
            }
        }
    }
}