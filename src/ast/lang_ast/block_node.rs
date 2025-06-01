use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;

#[derive(Debug)]
pub enum BlockNode {
    Item(Vec<BlockItemNode>)
}

impl GenerateTackyInstructions<()> for BlockNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        let BlockNode::Item(items) = self;
        items.iter().for_each(|item| item.to_tacky(tacky_instructions));
    }
}

impl AstDebugPrinter for BlockNode {
    fn debug_visit(&self) {
        let BlockNode::Item(items) = self;
        items.iter().for_each(|item| item.debug_visit());
    }
}