use std::collections::HashMap;
use crate::ast::lang_ast::function_node::FunctionNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_trait::CheckGotoLabel;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;

impl ResolveVarExprLabel for FunctionNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let FunctionNode::FunctionDef { func_name: _, block_item } = self;
        for item in block_item {
            item.resolve(var_map, label_map)?
        }
        Ok(())
    }
}

impl CheckGotoLabel for FunctionNode {
    fn check_goto_label(&mut self, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let FunctionNode::FunctionDef { func_name: _, block_item } = self;
        for item in block_item {
            item.check_goto_label(label_map)?
        }
        Ok(())
    }
}