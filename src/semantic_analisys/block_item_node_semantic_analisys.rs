use std::collections::HashMap;
use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_trait::CheckGotoLabel;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;

impl ResolveVarExprLabel for BlockItemNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        if let BlockItemNode::Statement(statement_node) = self {
            return statement_node.resolve(var_map, label_map);
        } else if let BlockItemNode::Declaration(declaration_node) = self {
            return declaration_node.resolve(var_map, label_map);
        }
        Ok(())
    }
}

impl CheckGotoLabel for BlockItemNode {
    fn check_goto_label(&mut self, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        if let BlockItemNode::Statement(statement_node) = self {
            return statement_node.check_goto_label(label_map)
        }
        Ok(())
    }
}