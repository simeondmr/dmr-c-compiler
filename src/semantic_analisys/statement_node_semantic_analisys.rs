use std::collections::HashMap;
use crate::ast::lang_ast::statement_node::StatementNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExpr;

impl ResolveVarExpr for StatementNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            StatementNode::ReturnStmt(expr) => expr.resolve(var_map),
            StatementNode::Expr(expr) => expr.resolve(var_map),
            StatementNode::EmptyStmt => { 
                // Note: nothing to do
                Ok(())
            }
        }
    }
}