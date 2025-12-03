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
use crate::ast::asm_ast::asm_binary_operator_node::AsmBinaryOperatorNode;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::ast::asm_ast::asm_unary_operator_node::AsmUnaryOperatorNode;
use crate::ast::lang_ast::lang_ast_visit_trait::AstDebugPrinter;

#[derive(Clone)]
#[allow(dead_code)]
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
    DeallocateStack(u64),
    Cmp(OperandAsmNode, OperandAsmNode),
    Jmp(u32),
    JmpCC {
        condition_code: ConditionCode,
        jmp_label_target: u32
    },
    Set {
        condition_code: ConditionCode,
        dest: OperandAsmNode
    },
    Inc(OperandAsmNode),
    Dec(OperandAsmNode),
    Push(OperandAsmNode),
    Call {
        func_name: String,
        has_body: bool
    },
    LinuxExitSyscall,
    Label(u32),
    Ret
}

#[derive(Clone)]
#[derive(Debug)]
pub enum ConditionCode {
    E,
    Ne,
    G,
    Ge,
    L,
    Le
}

impl ConditionCode {
    pub fn code(&self) -> &str {
        match self {
            ConditionCode::E => "e",
            ConditionCode::Ne => "ne",
            ConditionCode::G => "g",
            ConditionCode::Ge => "ge",
            ConditionCode::L => "l",
            ConditionCode::Le => "le"
        }
    }
}

impl AstDebugPrinter for InstructionAsmNode {
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
            InstructionAsmNode::DeallocateStack(size) => println!("DeallocateStack {}", *size),
            InstructionAsmNode::JmpCC { condition_code, jmp_label_target} => println!("JmpCC{:?} .l{}", condition_code, jmp_label_target),
            InstructionAsmNode::Cmp(val0, val1) => println!("Cmp {:?} {:?}", val0, val1),
            InstructionAsmNode::Jmp(jmp_label_target) => println!("Jmp .l{}", jmp_label_target),
            InstructionAsmNode::Set { condition_code, dest } => println!("set{:?} {:?}", condition_code, dest),
            InstructionAsmNode::Inc(operand) => println!("inc {:?}", operand),
            InstructionAsmNode::Dec(operand) => println!("dec {:?}", operand),
            InstructionAsmNode::Push(operand) => println!("push {:?}", operand),
            InstructionAsmNode::Call { func_name, has_body} => println!("call {func_name} {has_body}"),
            InstructionAsmNode::LinuxExitSyscall => println!("LinuxExitSyscall"),
            InstructionAsmNode::Label(index) => println!(".l{}:", index),
            InstructionAsmNode::Ret => println!("Ret")

        }
    }
}