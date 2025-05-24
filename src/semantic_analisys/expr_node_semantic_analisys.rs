use std::collections::HashMap;
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExpr;

impl ResolveVarExpr for ExprNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            ExprNode::Constant(_) => Ok(()),
            ExprNode::Unary { unary_operator: _, expr } => expr.resolve(var_map),
            ExprNode::Binary { binary_operator: _, left_expr, right_expr } => {
                left_expr.resolve(var_map)?;
                right_expr.resolve(var_map)
            },
            ExprNode::Assignment { assignment_type: _, dest, expr } => {
                if !matches!(&**dest, ExprNode::Var { .. }) {
                    eprintln!("Error: expected var in lvalue");
                    return Err(CompilerErrors::SemanticError);
                }
                dest.resolve(var_map)?;
                expr.resolve(var_map)
            },
            ExprNode::PrePostOperator { pre_post_operator_type: _, identifier} => identifier.resolve(var_map),
            ExprNode::Conditional { condition, true_expr, false_expr } => {
                condition.resolve(var_map)?;
                true_expr.resolve(var_map)?;
                false_expr.resolve(var_map)
            }
            ExprNode::Var { var_name, var_name_index: _} => {
                if let Some(var_name_index) = var_map.get(var_name) {
                    *self = ExprNode::Var { var_name: var_name.to_string(), var_name_index: *var_name_index };
                } else {
                    eprintln!("Error: undeclared var {}", var_name);
                    return Err(CompilerErrors::SemanticError);
                }
                Ok(())
            }
        }
    }
}