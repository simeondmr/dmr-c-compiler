use std::collections::HashMap;
use crate::ast::lang_ast::block_node::BlockNode;
use crate::ast::lang_ast::statement_node::LoopLabels;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_break_continue_trait::{CheckGotoLabelBreakContinue};
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

impl CheckGotoLabelBreakContinue for BlockNode {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels) -> Result<(), CompilerErrors> {
        let BlockNode::Item(items) = self;
        for item in items {
            item.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels)?
        }
        Ok(())
    }
}