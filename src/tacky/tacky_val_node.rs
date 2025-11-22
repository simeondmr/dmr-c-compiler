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

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::LazyLock;
use crate::ast::asm_ast::asm_operand_node::OperandAsmNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

#[derive(Clone, Debug)]
pub enum ValTackyNode {
    Constant(i32),
    Var(u32),
    // Note: we need this Empty marker only for "fix" the match in the expr_node.rs in order to covering Assign(expr, expr), also if this case will not pass the semantic analisys
    Empty
}

impl GenerateAsm<OperandAsmNode> for ValTackyNode {
    fn to_asm(&self) -> OperandAsmNode {
        match self {
            ValTackyNode::Constant(value) => OperandAsmNode::Imm(*value),
            ValTackyNode::Var(identifier) => OperandAsmNode::Pseudo(*identifier),
            _ => {
                // Note that this case can't happen
                OperandAsmNode::Empty
            }
        }
    }
}

impl TackyVisitDebug for ValTackyNode {
    fn visit_debug(&self) {
        match self {
            ValTackyNode::Constant(value) => println!("Constant {}", value),
            ValTackyNode::Var(name) => println!("Var t{}", name),
            _ => {
                // Note: nothing to do
            }
        }
    }
}

static COUNTER: LazyLock<AtomicU32> = LazyLock::new(|| AtomicU32::new(0));

pub struct TemporaryVar;

impl TemporaryVar {
    pub fn generate() -> u32 {
        COUNTER.fetch_add(1, Ordering::Relaxed) + 1
    }

    pub fn start_from(value: u32) {
        COUNTER.store(value, Ordering::Relaxed);
    }

    pub fn reset() {
        COUNTER.store(0, Ordering::Relaxed);
    }
}