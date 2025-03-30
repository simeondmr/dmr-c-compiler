use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::tacky::tacky_function_node::FunctionTackyNode;
use crate::tacky::tacky_visit_trait::{GenerateAsm, TackyVisitDebug};

pub enum ProgramTackyNode {
    ProgramDef(FunctionTackyNode)
}

impl TackyVisitDebug for ProgramTackyNode {
    fn visit_debug(&self) {
        let ProgramTackyNode::ProgramDef(function) = self;
        function.visit_debug();
    }
}

impl GenerateAsm<AsmProgramNode> for ProgramTackyNode {
    fn to_asm(&self) -> AsmProgramNode {
        let ProgramTackyNode::ProgramDef(function) = self;
        AsmProgramNode::ProgramAsmDef(function.to_asm())
    }
}