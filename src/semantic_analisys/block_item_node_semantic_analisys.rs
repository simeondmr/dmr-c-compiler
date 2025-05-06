use std::collections::HashMap;
use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExpr;

impl ResolveVarExpr for BlockItemNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        if let BlockItemNode::Statement(statement_node) = self {
            return statement_node.resolve(var_map);
        } else if let BlockItemNode::Declaration(declaration_node) = self {
            return declaration_node.resolve(var_map);
        }
        Ok(())
    }
}