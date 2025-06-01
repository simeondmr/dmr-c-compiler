use std::collections::HashMap;
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::symbol_table::SymbolTable;

impl ResolveVarExprLabel for ExprNode {
    fn resolve(&mut self, symbol_table: &mut SymbolTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            ExprNode::Constant(_) => Ok(()),
            ExprNode::Unary { unary_operator: _, expr } => expr.resolve(symbol_table, label_map),
            ExprNode::Binary { binary_operator: _, left_expr, right_expr } => {
                left_expr.resolve(symbol_table, label_map)?;
                right_expr.resolve(symbol_table, label_map)
            },
            ExprNode::Assignment { assignment_type: _, dest, expr } => {
                if !matches!(&**dest, ExprNode::Var { .. }) {
                    eprintln!("Error: expected var in lvalue");
                    return Err(CompilerErrors::SemanticError);
                }
                dest.resolve(symbol_table, label_map)?;
                expr.resolve(symbol_table, label_map)
            },
            ExprNode::PrePostOperator { pre_post_operator_type: _, identifier} => identifier.resolve(symbol_table, label_map),
            ExprNode::Conditional { condition, true_expr, false_expr } => {
                condition.resolve(symbol_table, label_map)?;
                true_expr.resolve(symbol_table, label_map)?;
                false_expr.resolve(symbol_table, label_map)
            },
            ExprNode::Var { var_name, var_name_index } => {
                if let Some(new_var_name_index) = symbol_table.var_index(var_name) {
                    *var_name_index = new_var_name_index;
                } else {
                    eprintln!("Error: undeclared var {}", var_name);
                    return Err(CompilerErrors::SemanticError);
                }
                Ok(())
            }
        }
    }
}