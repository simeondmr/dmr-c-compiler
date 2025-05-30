use std::collections::HashMap;
use std::sync::Mutex;
use crate::ast::lang_ast::statement_node::StatementNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_trait::CheckGotoLabel;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::tacky::label_gen::{ LabelGen, LABEL_GEN_SINGLETON };

impl ResolveVarExprLabel for StatementNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            StatementNode::IfStmt { condition, stmt, else_stmt } =>  {
                condition.resolve(var_map, label_map)?;
                stmt.resolve(var_map, label_map)?;
                if let Some(else_stmt_unwrapped) = else_stmt {
                    else_stmt_unwrapped.resolve(var_map, label_map)?;
                }
                Ok(())
            },
            StatementNode::ReturnStmt(expr) => expr.resolve(var_map, label_map),
            StatementNode::Goto { label_name: _, label_name_index: _ } => {
                /* Nothing to do because at this point the label may not have been declared yet */
                Ok(())
            },
            StatementNode::LabelStmt { label_name, label_name_index, stmt} => {
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let mut labelgen_singleton = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap();
                let new_label_index = labelgen_singleton.gen();
                label_map.insert(label_name.to_string(), new_label_index);
                *label_name_index = new_label_index;
                stmt.resolve(var_map, label_map)
            },
            StatementNode::Expr(expr) => expr.resolve(var_map, label_map),
            StatementNode::EmptyStmt => {
                // Note: nothing to do
                Ok(())
            }
        }
    }
}

impl CheckGotoLabel for StatementNode {
    fn check_goto_label(&mut self, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            StatementNode::IfStmt { condition: _, stmt, else_stmt } =>  {
                stmt.check_goto_label(label_map)?;
                if let Some(else_stmt_unwrapped) = else_stmt {
                    else_stmt_unwrapped.check_goto_label(label_map)?;
                }
                Ok(())
            },
            StatementNode::ReturnStmt(_) => {
                // Nothing to do
                Ok(())
            },
            StatementNode::Goto { label_name, label_name_index } => {
                if let Some(new_label_name_index) = label_map.get(label_name) {
                    *label_name_index = *new_label_name_index;
                    return Ok(())
                }
                eprintln!("Error: missing declaration of label {}", label_name);
                Err(CompilerErrors::SemanticError)
            },
            StatementNode::LabelStmt { label_name: _, label_name_index: _, stmt} => stmt.check_goto_label(label_map),
            StatementNode::Expr(_) => {
                // Nothing to do
                Ok(())
            },
            StatementNode::EmptyStmt => {
                // Note: nothing to do
                Ok(())
            }
        }
    }
}