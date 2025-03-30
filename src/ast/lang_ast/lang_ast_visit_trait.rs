use crate::tacky::tacky_instruction_node::InstructionTackyNode;

pub trait AstDebugPrinter {
    fn debug_visit(&self);
}

pub trait GenerateTacky<T> {
    fn to_tacky(&self) -> T;
}

pub trait GenerateTackyInstructions<T> {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> T;
}