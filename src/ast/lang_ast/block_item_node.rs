use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::ast::lang_ast::statement_node::StatementNode;
use crate::tacky::tacky_instruction_node::InstructionTackyNode;

#[derive(Debug)]
pub enum BlockItemNode {
    Statement(StatementNode),
    Declaration(DeclarationNode)
}

impl GenerateTackyInstructions<()> for BlockItemNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        if let BlockItemNode::Statement(statement_node) = self {
            statement_node.to_tacky(tacky_instructions)
        } else if let BlockItemNode::Declaration(declaration_node) = self {
            declaration_node.to_tacky(tacky_instructions);
        }
    }
}

impl AstDebugPrinter for BlockItemNode {
    fn debug_visit(&self) {
        if let BlockItemNode::Statement(statement_node) = self {
            statement_node.debug_visit();
        } else if let BlockItemNode::Declaration(declaration_node) = self {
            declaration_node.debug_visit();
        }
    }
}