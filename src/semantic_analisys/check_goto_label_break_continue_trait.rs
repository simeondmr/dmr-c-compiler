use std::collections::HashMap;
use crate::ast::lang_ast::statement_node::LoopLabels;
use crate::errors::errors::CompilerErrors;

/// This trait need to perform an AST pass in order to check if all goto labels are declared
pub trait CheckGotoLabelBreakContinue {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels, case_list: &mut Option<&mut HashMap<i32, u32>>, default_label: &mut Option<u32>) -> Result<(), CompilerErrors>;
}