use std::collections::HashMap;
use crate::ast::lang_ast::function_node::FunctionNode;
use crate::ast::lang_ast::statement_node::LoopLabels;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_break_continue_trait::{CheckGotoLabelBreakContinue};
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::symbol_table::SymbolTable;

impl ResolveVarExprLabel for FunctionNode {
    fn resolve(&mut self, symbol_table: &mut SymbolTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let FunctionNode::FunctionDef { func_name: _, block } = self;
        block.resolve(symbol_table, label_map)?;
        Ok(())
    }
}

impl CheckGotoLabelBreakContinue for FunctionNode {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels) -> Result<(), CompilerErrors> {
        let FunctionNode::FunctionDef { func_name: _, block } = self;
        block.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels)?;
        Ok(())
    }
}