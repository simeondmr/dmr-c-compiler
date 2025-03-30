use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

#[derive(Clone, Debug)]
pub enum ValTackyNode {
    Constant(i32),
    Var(u32)
}

impl TackyVisitDebug for ValTackyNode {
    fn visit_debug(&self) {
        match self {
            ValTackyNode::Constant(value) => println!("Constant {}", value),
            ValTackyNode::Var(name) => println!("Var t{}", name)
        }
    }
}

impl GenerateAsm<OperandAsmNode> for ValTackyNode {
    fn to_asm(&self) -> OperandAsmNode {
        match self {
            ValTackyNode::Constant(value) => OperandAsmNode::Imm(*value),
            ValTackyNode::Var(identifier) => OperandAsmNode::Pseudo(*identifier)
        }
    }
}

static COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));

pub struct TemporaryVar;

impl TemporaryVar {
    pub fn generate() -> u32 {
        COUNTER.fetch_add(1, Ordering::Relaxed) + 1
    }
}