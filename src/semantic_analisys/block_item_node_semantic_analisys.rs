use std::collections::HashMap;
use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::ast::lang_ast::statement_node::LoopLabels;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_break_continue_trait::{CheckGotoLabelBreakContinue};
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::symbol_table::SymbolTable;

impl ResolveVarExprLabel for BlockItemNode {
    fn resolve(&mut self, symbol_table: &mut SymbolTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        if let BlockItemNode::Statement(statement_node) = self {
            return statement_node.resolve(symbol_table, label_map);
        } else if let BlockItemNode::Declaration(declaration_node) = self {
            return declaration_node.resolve(symbol_table, label_map);
        }
        Ok(())
    }
}

impl CheckGotoLabelBreakContinue for BlockItemNode {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels, case_map: &mut Option<&mut HashMap<i32, u32>>, default_label: &mut Option<u32>) -> Result<(), CompilerErrors> {
        if let BlockItemNode::Statement(statement_node) = self {
            return statement_node.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label)
        }
        Ok(())
    }
}