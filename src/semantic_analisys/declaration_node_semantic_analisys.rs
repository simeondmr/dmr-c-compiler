use std::collections::HashMap;
use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::symbol_table::SymbolTable;
use crate::tacky::tacky_val_node::TemporaryVar;

impl ResolveVarExprLabel for DeclarationNode {
    fn resolve(&mut self, symbol_table: &mut SymbolTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let DeclarationNode::VariableDeclaration { var_name, var_name_index, init } = self;
        let new_var_name_index = TemporaryVar::generate();
        symbol_table.new_variable(var_name.to_string(), new_var_name_index)?;
        *var_name_index = new_var_name_index;
        if let Some(expr) = init {
            return expr.resolve(symbol_table, label_map);
        }
        Ok(())
    }
}