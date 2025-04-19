use std::collections::VecDeque;
use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_registers_node::Reg;
use crate::tacky::binary_operator_tacky_node::BinaryOperatorTackyNode;
use crate::tacky::tacky_unary_operator_node::UnaryOperatorTackyNode;
use crate::tacky::tacky_val_node::ValTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, GenerateAsmInstruction, TackyVisitDebug};

pub enum InstructionTackyNode {
    Return(ValTackyNode),
    Unary {
        unary_operator: UnaryOperatorTackyNode,
        src: ValTackyNode,
        dest: ValTackyNode
    },
    Binary {
        binary_operator: BinaryOperatorTackyNode,
        left_expr: ValTackyNode,
        right_expr: ValTackyNode,
        dest: ValTackyNode
    }
}

impl TackyVisitDebug for InstructionTackyNode {
    fn visit_debug(&self) {
        match self {
            InstructionTackyNode::Return(val) => {
                println!("Return(");
                val.visit_debug();
                println!(")");
            },
            InstructionTackyNode::Unary { unary_operator, src, dest} => {
                println!("UnaryOperator(");
                unary_operator.visit_debug();
                print!("src: ");
                src.visit_debug();
                print!("dest: ");
                dest.visit_debug();
                println!(")");
            }
            InstructionTackyNode::Binary { binary_operator, left_expr, right_expr, dest } => {
                println!("BinaryOperator(");
                binary_operator.visit_debug();
                println!("Left expression: ");
                left_expr.visit_debug();
                println!("Right expression: ");
                right_expr.visit_debug();
                println!("Dest expression: ");
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
                asm_instructions.push_back(InstructionAsmNode::Mov{ src: val.to_asm(), dest: OperandAsmNode::Register(Reg::AX) });
                asm_instructions.push_back(InstructionAsmNode::Ret);
            },
            InstructionTackyNode::Unary {unary_operator, src, dest } => {
                let asm_dest_val = dest.to_asm();
                asm_instructions.push_back(InstructionAsmNode::Mov { src: src.to_asm(), dest: asm_dest_val.clone() });
                asm_instructions.push_back(InstructionAsmNode::Unary { operator: unary_operator.to_asm(), operand: asm_dest_val });
            },
            InstructionTackyNode::Binary { binary_operator, left_expr, right_expr, dest } if matches!(binary_operator, BinaryOperatorTackyNode::Divide | BinaryOperatorTackyNode::Remainder) => {
                asm_instructions.push_back(InstructionAsmNode::Mov { src: left_expr.to_asm(), dest: OperandAsmNode::Register(Reg::AX) });
                asm_instructions.push_back(InstructionAsmNode::Cdq);
                asm_instructions.push_back(InstructionAsmNode::Idiv(right_expr.to_asm()));

                if let BinaryOperatorTackyNode::Divide = binary_operator {
                    asm_instructions.push_back(InstructionAsmNode::Mov { src: OperandAsmNode::Register(Reg::AX), dest: dest.to_asm() });
                } else {
                    asm_instructions.push_back(InstructionAsmNode::Mov { src: OperandAsmNode::Register(Reg::DX), dest: dest.to_asm() });
                }
            },
            InstructionTackyNode::Binary { binary_operator, left_expr, right_expr, dest } => {
                let asm_dest_val = dest.to_asm();
                asm_instructions.push_back(InstructionAsmNode::Mov { src: left_expr.to_asm(), dest: asm_dest_val.clone() });
                asm_instructions.push_back(InstructionAsmNode::Binary { operator: binary_operator.to_asm().unwrap(), src: right_expr.to_asm(), dest: asm_dest_val.clone() });
            }
        }
    }
}