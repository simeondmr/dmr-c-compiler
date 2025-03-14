use crate::ast::asm_ast::asm_instruction_node::InstructionAsmNode;

pub trait AstDebugPrinter {
    fn debug_visit(&self);
}

pub trait GenerateAsmAst<T> {
    fn to_asm_ast(&self) -> T;
}

pub trait GenerateListAsmInstructions {
    fn to_asm_ast(&self, asm_instructions: &mut Vec<InstructionAsmNode>);
}