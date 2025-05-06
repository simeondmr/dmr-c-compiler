use std::collections::HashMap;
use crate::ast::lang_ast::function_node::FunctionNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExpr;

impl ResolveVarExpr for FunctionNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let FunctionNode::FunctionDef { func_name: _, block_item } = self;
        for item in block_item {
            item.resolve(var_map)?
        }
        Ok(())
    }
}