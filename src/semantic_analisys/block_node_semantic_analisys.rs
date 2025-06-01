use std::collections::HashMap;
use crate::ast::lang_ast::block_node::BlockNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_trait::CheckGotoLabel;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::symbol_table::SymbolTable;

impl ResolveVarExprLabel for BlockNode {
    fn resolve(&mut self, symbol_table: &mut SymbolTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let BlockNode::Item(items) = self;
        symbol_table.push_block();
        for item in items {
            item.resolve(symbol_table, label_map)?;
        }
        symbol_table.pop_block();
        Ok(())
    }
}

impl CheckGotoLabel for BlockNode {
    fn check_goto_label(&mut self, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let BlockNode::Item(items) = self;
        for item in items {
            item.check_goto_label(label_map)?;
        }
        Ok(())
    }
}