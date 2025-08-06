use std::collections::HashMap;
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::ast::lang_ast::statement_node::LoopLabels;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_break_continue_trait::{ CheckGotoLabelBreakContinue };
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::symbol_table::SymbolTable;

impl ResolveVarExprLabel for ProgramNode {
    fn resolve(&mut self, symbol_table: &mut SymbolTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let ProgramNode::ProgramDef(function_def) = self;
        function_def.resolve(symbol_table, label_map)
    }
}

impl CheckGotoLabelBreakContinue for ProgramNode {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels) -> Result<(), CompilerErrors> {
        let ProgramNode::ProgramDef(function_def) = self;
        function_def.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels)
    }
}