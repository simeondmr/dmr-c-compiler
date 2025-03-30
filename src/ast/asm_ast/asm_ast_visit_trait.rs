use crate::codegen::stack_alloc_table::StackAllocTable;

/// This trait provides a method for visiting an assembly tree and print nodes for debugging
pub trait AstAsmDebugPrinter {
    fn debug_visit(&self);
}

/// This trait provides a method for replacing assembly pseudoregister with the effective stack address
pub trait AsmReplacingPseudoregisters {
    /// Replace a Pseudo register with the effective stack address using the StackAllocTable for the stack mapping
    ///
    /// # Arguments
    /// * stack_alloc_table - The stack allocation table for Pseudo registers memory mapping
    ///
    /// # Returns
    /// Return the function's stack offset
    fn replacing_pseudoregisters(&mut self, stack_alloc_table: &mut StackAllocTable) -> i32;
}

/// This trait provides a method for fixing the assembly tree instructions and add the stack allocation at the beginning of instructions
/// For example if this method meet an instructions like: movl [stack_operand], [stack_operand] it will fix them in order to avoid to have 2 memory operand
pub trait FixingInstruction {
    /// Fix the assembly instructions from the assembly tree
    /// # Argument
    /// * stack_offset - Number of bytes for stack allocation
    fn fixing_instructions(&mut self, stack_offset: i32);
}