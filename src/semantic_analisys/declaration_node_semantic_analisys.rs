use std::collections::HashMap;
use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::tacky::tacky_val_node::TemporaryVar;

impl ResolveVarExprLabel for DeclarationNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let DeclarationNode::VariableDeclaration { var_name, var_name_index, init } = self;
        if let Some(_) = var_map.get(var_name) {
            eprintln!("Error: redefinition of variable {}", var_name);
            return Err(CompilerErrors::SemanticError);
        }
        let new_var_name_index = TemporaryVar::generate();
        var_map.insert(var_name.to_string(), new_var_name_index);
        *var_name_index = new_var_name_index;
        if let Some(expr) = init {
            return expr.resolve(var_map, label_map);
        }
        Ok(())
    }
}