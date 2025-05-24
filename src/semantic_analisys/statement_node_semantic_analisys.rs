use std::collections::HashMap;
use crate::ast::lang_ast::statement_node::StatementNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExpr;

impl ResolveVarExpr for StatementNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            StatementNode::IfStmt { condition, stmt, else_stmt } =>  {
                condition.resolve(var_map)?;
                stmt.resolve(var_map)?;
                if let Some(else_stmt_unwrapped) = else_stmt {
                    else_stmt_unwrapped.resolve(var_map)?;
                }
                Ok(())
            },
            StatementNode::ReturnStmt(expr) => expr.resolve(var_map),
            StatementNode::Expr(expr) => expr.resolve(var_map),
            StatementNode::EmptyStmt => { 
                // Note: nothing to do
                Ok(())
            }
        }
    }
}