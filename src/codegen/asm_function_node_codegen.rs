use crate::ast::asm_ast::asm_function_node::FunctionAsmNode;
use crate::codegen::asm_codegen_trait::Codegen;

impl Codegen for FunctionAsmNode {
    fn codegen(&self) {
        let FunctionAsmNode::FunctionAsmDef(name, ref instructions) = self;
        println!(".globl {}", name);
        println!("{}:", name);
        println!("pushq %rbp");
        println!("movq %rsp, %rbp");
        instructions.iter().for_each(|instruction| instruction.codegen());
    }
}