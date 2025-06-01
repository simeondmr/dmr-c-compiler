use std::collections::HashMap;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::symbol_table::SymbolTable;

pub trait ResolveVarExprLabel {
    fn resolve(&mut self, var_symbol_table: &mut SymbolTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors>;
}