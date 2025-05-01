use std::collections::VecDeque;
use crate::ast::asm_ast::asm_instruction_node::{ConditionCode, InstructionAsmNode};
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
    },
    Jmp(u32),
    JmpIfZero {
        condition: ValTackyNode,
        jmp_label_target: u32
    },
    JmpIfNotZero {
        condition: ValTackyNode,
        jmp_label_target: u32
    },
    Copy {
        src: ValTackyNode,
        dest: ValTackyNode
    },
    Label(u32)
}

impl GenerateAsmInstruction<()> for InstructionTackyNode {
    fn to_asm(&self, asm_instructions: &mut VecDeque<InstructionAsmNode>) -> () {
        match self {
            InstructionTackyNode::Return(val) =>  {
                asm_instructions.push_back(InstructionAsmNode::Mov{ src: val.to_asm(), dest: OperandAsmNode::Register(Reg::AX) });
                asm_instructions.push_back(InstructionAsmNode::Ret);
            },
            InstructionTackyNode::Unary { unary_operator, src, dest } if matches!(unary_operator, UnaryOperatorTackyNode::Not) => {
                asm_instructions.push_back(InstructionAsmNode::Cmp(OperandAsmNode::Imm(0), src.to_asm()));
                asm_instructions.push_back(InstructionAsmNode::Mov { src: OperandAsmNode::Imm(0), dest: dest.to_asm() });
                asm_instructions.push_back(InstructionAsmNode::Set { condition_code: ConditionCode::E, dest: dest.to_asm() });
            },
            InstructionTackyNode::Unary { unary_operator, src, dest } => {
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
            InstructionTackyNode::Binary { binary_operator, left_expr, right_expr, dest } if matches!(binary_operator, BinaryOperatorTackyNode::Equal | BinaryOperatorTackyNode::NotEqual | BinaryOperatorTackyNode::LessThan | BinaryOperatorTackyNode::LessThanOrEqual | BinaryOperatorTackyNode::GreaterThan | BinaryOperatorTackyNode::GreaterThanOrEqual) => {
                asm_instructions.push_back(InstructionAsmNode::Cmp(right_expr.to_asm(), left_expr.to_asm()));
                asm_instructions.push_back(InstructionAsmNode::Mov { src: OperandAsmNode::Imm(0), dest: dest.to_asm() });
                asm_instructions.push_back(InstructionAsmNode::Set { condition_code: binary_operator.to_condition_code().unwrap(), dest: dest.to_asm() });
            },
            InstructionTackyNode::Binary { binary_operator, left_expr, right_expr, dest } => {
                let asm_dest_val = dest.to_asm();
                asm_instructions.push_back(InstructionAsmNode::Mov { src: left_expr.to_asm(), dest: asm_dest_val.clone() });
                asm_instructions.push_back(InstructionAsmNode::Binary { operator: binary_operator.to_asm().unwrap(), src: right_expr.to_asm(), dest: asm_dest_val.clone() });
            },
            InstructionTackyNode::Jmp(jmp_label_target) => asm_instructions.push_back(InstructionAsmNode::Jmp(*jmp_label_target)),
            InstructionTackyNode::JmpIfZero { condition, jmp_label_target} => {
                asm_instructions.push_back(InstructionAsmNode::Cmp(OperandAsmNode::Imm(0), condition.to_asm()));
                asm_instructions.push_back(InstructionAsmNode::JmpCC { condition_code: ConditionCode::E, jmp_label_target: *jmp_label_target });
            },
            InstructionTackyNode::JmpIfNotZero { condition, jmp_label_target} => {
                asm_instructions.push_back(InstructionAsmNode::Cmp(OperandAsmNode::Imm(0), condition.to_asm()));
                asm_instructions.push_back(InstructionAsmNode::JmpCC { condition_code: ConditionCode::Ne, jmp_label_target: *jmp_label_target });
            },
            InstructionTackyNode::Copy { src, dest} => asm_instructions.push_back(InstructionAsmNode::Mov { src: src.to_asm(), dest: dest.to_asm() }),
            InstructionTackyNode::Label(index) => asm_instructions.push_back(InstructionAsmNode::Label(*index))
        }
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
            },
            InstructionTackyNode::Binary { binary_operator, left_expr, right_expr, dest } => {
                println!("BinaryOperator(");
                binary_operator.visit_debug();
                print!("Left expression: ");
                left_expr.visit_debug();
                print!("Right expression: ");
                right_expr.visit_debug();
                print!("Dest expression: ");
                dest.visit_debug();
                println!(")");
            },
            InstructionTackyNode::Jmp(jmp_label_target) => println!("Jmp l{}", jmp_label_target),
            InstructionTackyNode::JmpIfZero { condition, jmp_label_target } => {
                condition.visit_debug();
                println!("JmpIfZero L{}", jmp_label_target);
            },
            InstructionTackyNode::JmpIfNotZero { condition, jmp_label_target } => {
                condition.visit_debug();
                println!("JmpIfNotZero l{}", jmp_label_target);
            },
            InstructionTackyNode::Copy {src, dest} => {
                println!("Copy(");
                print!("src: ");
                src.visit_debug();
                print!("dest: ");
                dest.visit_debug();
                println!(")");
            }
            InstructionTackyNode::Label(value) => {
                println!("l{}:", value);
            }
        }
    }
}