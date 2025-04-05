use std::fs::File;
use std::path::Path;
use crate::ast::asm_ast::asm_ast_visit_trait::{AsmReplacingPseudoregisters, FixingInstruction};
use crate::ast::asm_ast::asm_program_node::AsmProgramNode;
use crate::codegen::stack_alloc_table::StackAllocTable;
use crate::codegen::asm_codegen_trait::Codegen;

pub struct CodegenCore<'a> {
    output_path : &'a Path
}

impl <'a> CodegenCore<'a> {
    pub fn new(output_path: &Path) -> CodegenCore {
        CodegenCore {
            output_path,
        }
    }

    pub fn codegen(&self, asm_ast: &mut AsmProgramNode)-> std::io::Result<()>  {
        let mut stack_alloc_table = StackAllocTable::new();
        let stack_offset = asm_ast.replacing_pseudoregisters(&mut stack_alloc_table);
        asm_ast.fixing_instructions(stack_offset);
        let mut output_file = File::create(self.output_path)?;
        asm_ast.codegen(&mut output_file)
    }
}