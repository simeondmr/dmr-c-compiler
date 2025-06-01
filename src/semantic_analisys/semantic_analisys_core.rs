use std::collections::HashMap;
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_trait::CheckGotoLabel;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::symbol_table::SymbolTable;

pub fn semantic_analisys(program_node: &mut ProgramNode) -> Result<(), CompilerErrors> {
    let mut symbol_table = SymbolTable::new();
    let mut label_map: HashMap<String, u32> = HashMap::new();
    program_node.resolve(&mut symbol_table, &mut label_map)?;
    program_node.check_goto_label(&mut label_map)
}