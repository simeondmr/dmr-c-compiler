use std::fmt::Debug;
use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky, GenerateTackyInstructions};
use crate::tacky::tacky_function_node::FunctionTackyNode;

#[derive(Debug)]
pub enum FunctionNode {
    FunctionDef {
        func_name: String, 
        block_item: Vec<BlockItemNode> 
    }
}

impl GenerateTacky<FunctionTackyNode> for FunctionNode {
    fn to_tacky(&self) -> FunctionTackyNode {
        let FunctionNode::FunctionDef { func_name, block_item } = self;
        let mut tacky_instructions = Vec::new();
        block_item.into_iter().for_each(|item| item.to_tacky(&mut tacky_instructions));
        FunctionTackyNode::FunctionDef { func_name: func_name.to_string(), tacky_instructions }
    }
}

impl AstDebugPrinter for FunctionNode {
    fn debug_visit(&self) {
        let FunctionNode::FunctionDef { func_name, block_item } = self;
        println!("Function(");
        println!("name=\"{}\"", func_name);
        block_item.into_iter().for_each(|item| item.debug_visit());
        println!(")");
    }
}