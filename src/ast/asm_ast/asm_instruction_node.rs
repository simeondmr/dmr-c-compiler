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
            InstructionAsmNode::JmpCC { condition_code, jmp_label_target} => println!("JmpCC{:?} .l{}", condition_code, jmp_label_target),
            InstructionAsmNode::Cmp(val0, val1) => println!("Cmp {:?} {:?}", val0, val1),
            InstructionAsmNode::Jmp(jmp_label_target) => println!("Jmp .l{}", jmp_label_target),
            InstructionAsmNode::Set { condition_code, dest } => println!("set{:?} {:?}", condition_code, dest),
            InstructionAsmNode::Label(index) => println!(".l{}:", index),
            InstructionAsmNode::Ret => println!("Ret")

        }
    }
}