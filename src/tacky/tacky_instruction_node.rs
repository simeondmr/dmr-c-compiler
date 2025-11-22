// dmr C compiler
// Copyright (C) 2025  Simeon Tornabene
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, see <https://www.gnu.org/licenses/>.
use std::collections::VecDeque;
use crate::ast::asm_ast::asm_instruction_node::{ConditionCode, InstructionAsmNode};
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode::Register;
use crate::ast::asm_ast::asm_registers_node::{RaxReg, RcxReg, Reg};
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
    Increment(ValTackyNode),
    Decrement(ValTackyNode),
    Copy {
        src: ValTackyNode,
        dest: ValTackyNode
    },
    FuncCall {
        func_name: String,
        args: Vec<ValTackyNode>,
        ret_val: ValTackyNode,
        has_body: bool
    },
    Label(u32)
}

impl GenerateAsmInstruction<()> for InstructionTackyNode {
    fn to_asm(&self, asm_instructions: &mut VecDeque<InstructionAsmNode>) -> () {
        match self {
            InstructionTackyNode::Return(val) =>  {
                asm_instructions.push_back(InstructionAsmNode::Mov{ src: val.to_asm(), dest: OperandAsmNode::Register(Reg::AX(RaxReg::EAX)) });
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
                asm_instructions.push_back(InstructionAsmNode::Mov { src: left_expr.to_asm(), dest: OperandAsmNode::Register(Reg::AX(RaxReg::EAX)) });
                asm_instructions.push_back(InstructionAsmNode::Cdq);
                asm_instructions.push_back(InstructionAsmNode::Idiv(right_expr.to_asm()));
                if let BinaryOperatorTackyNode::Divide = binary_operator {
                    asm_instructions.push_back(InstructionAsmNode::Mov { src: OperandAsmNode::Register(Reg::AX(RaxReg::EAX)), dest: dest.to_asm() });
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
            InstructionTackyNode::Increment(expr) => asm_instructions.push_back(InstructionAsmNode::Inc(expr.to_asm())),
            InstructionTackyNode::Decrement(expr) => asm_instructions.push_back(InstructionAsmNode::Dec(expr.to_asm())),
            InstructionTackyNode::Copy { src, dest} => asm_instructions.push_back(InstructionAsmNode::Mov { src: src.to_asm(), dest: dest.to_asm() }),
            InstructionTackyNode::Label(index) => asm_instructions.push_back(InstructionAsmNode::Label(*index)),
            InstructionTackyNode::FuncCall { func_name, args, ret_val, has_body } => {
                let arg_registers = [Reg::DI, Reg::SI, Reg::DX, Reg::CX(RcxReg::ECX), Reg::R8, Reg::R9];
                let registers_len = arg_registers.len();
                let mut padding = 0;
                let param_stack_len = (args.len().saturating_sub(arg_registers.len())) as u64;
                if param_stack_len > 0 && param_stack_len % 2 != 0 {
                    padding = 8;
                    asm_instructions.push_back(InstructionAsmNode::AllocateStack(padding));
                }
                //Note: put args in registers
                for i in 0..args.len() as u64 - param_stack_len {
                    asm_instructions.push_back(InstructionAsmNode::Mov { src: args[i as usize].to_asm(), dest: OperandAsmNode::Register(arg_registers[i as usize].clone()) });
                }
                //Note: put remaining args into stack
                for i in (0..param_stack_len).rev() {
                    let arg_asm = args[registers_len + i  as usize].to_asm();
                    if let OperandAsmNode::Imm(_) = arg_asm {
                        asm_instructions.push_back(InstructionAsmNode::Push(arg_asm));
                    } else {
                        asm_instructions.push_back(InstructionAsmNode::Mov { src: arg_asm, dest: Register(Reg::AX(RaxReg::EAX)) });
                        asm_instructions.push_back(InstructionAsmNode::Push(Register(Reg::AX(RaxReg::RAX))));
                    }
                }
                asm_instructions.push_back(InstructionAsmNode::Call { func_name: func_name.clone(), has_body: *has_body });
                let byte_to_remove_sp = padding as u64 + 8 * param_stack_len;
                if byte_to_remove_sp > 0 {
                    asm_instructions.push_back(InstructionAsmNode::DeallocateStack(byte_to_remove_sp));
                }
                asm_instructions.push_back(InstructionAsmNode::Mov { src: Register(Reg::AX(RaxReg::EAX)), dest: ret_val.to_asm() });
            }
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
            },
            InstructionTackyNode::Increment(expr) => {
                println!("Increment(");
                expr.visit_debug();
                println!(")");
            },
            InstructionTackyNode::Decrement(expr) => {
                println!("Decrement(");
                expr.visit_debug();
                println!(")");
            },
            InstructionTackyNode::Label(value) => {
                println!("l{}:", value);
            },
            InstructionTackyNode::FuncCall { func_name, args, ret_val, has_body: _ } => {
                println!("FuncCall {} {:?} {:?}", func_name, args, ret_val)
            }
        }
    }
}