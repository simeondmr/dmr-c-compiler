// dmr C compiler
// Copyright (C) 2025  Simeon Tornabene
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 2
// of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, see <https://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::sync::Mutex;
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::ast::lang_ast::statement_node::{ForInit, LoopLabels, StatementNode};
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_break_continue_trait::{CheckGotoLabelBreakContinue};
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::identifier_table::IdentifierTable;
use crate::tacky::label_gen::{ LabelGen, LABEL_GEN_SINGLETON };

impl ResolveVarExprLabel for StatementNode {
    fn resolve(&mut self, identifier_table: &mut IdentifierTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            StatementNode::IfStmt { condition, stmt, else_stmt } =>  {
                condition.resolve(identifier_table, label_map)?;
                stmt.resolve(identifier_table, label_map)?;
                if let Some(else_stmt_unwrapped) = else_stmt {
                    else_stmt_unwrapped.resolve(identifier_table, label_map)?;
                }
                Ok(())
            },
            StatementNode::WhileStmt { condition, stmt, loop_labels: _ } => {
                condition.resolve(identifier_table, label_map)?;
                Ok(stmt.resolve(identifier_table, label_map)?)
            },
            StatementNode::DoWhileStmt { condition, stmt, loop_labels: _} => {
                condition.resolve(identifier_table, label_map)?;
                Ok(stmt.resolve(identifier_table, label_map)?)
            },
            StatementNode::ForStmt { init , condition, post_expr, stmt, loop_labels: _ } => {
                init.resolve(identifier_table, label_map)?;
                if let Some(condition_node) = condition {
                    condition_node.resolve(identifier_table, label_map)?;
                }
                if let Some(post_expr_node) = post_expr {
                    post_expr_node.resolve(identifier_table, label_map)?;
                }
                Ok(stmt.resolve(identifier_table, label_map)?)
            },
            StatementNode::SwitchStmt { condition, stmt, loop_labels: _, cases_map: _, default_stmt_label: _ } => {
                condition.resolve(identifier_table, label_map)?;
                Ok(stmt.resolve(identifier_table, label_map)?)
            },
            StatementNode::CaseStmt { label: _, value, stmt } => {
                if let ExprNode::Constant(_) = value {
                    return Ok(stmt.resolve(identifier_table, label_map)?)
                }
                eprintln!("Error: case value must be a constant value");
                Err(CompilerErrors::SemanticError)
            }
            StatementNode::DefaultStmt { label: _, stmt} => Ok(stmt.resolve(identifier_table, label_map)?),
            StatementNode::ReturnStmt(expr) => expr.resolve(identifier_table, label_map),
            StatementNode::Goto { label_name: _, label_name_index: _ } => {
                /* Nothing to do because at this point the label may not have been declared yet */
                Ok(())
            },
            StatementNode::BreakStmt(_) => {
                /* Nothing to do */
                Ok(())
            },
            StatementNode::ContinueStmt(_) => {
                /* Nothing to do */
                Ok(())
            },
            StatementNode::LabelStmt { label_name, label_name_index, stmt} => {
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let mut labelgen_singleton = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap();
                let new_label_index = labelgen_singleton.gen();
                label_map.insert(label_name.to_string(), new_label_index);
                *label_name_index = new_label_index;
                stmt.resolve(identifier_table, label_map)
            },
            StatementNode::Compound(block_node) => block_node.resolve(identifier_table, label_map),
            StatementNode::Expr(expr) => expr.resolve(identifier_table, label_map),
            StatementNode::EmptyStmt => {
                // Note: nothing to do
                Ok(())
            }
        }
    }
}

impl ResolveVarExprLabel for ForInit {
    fn resolve(&mut self, identifier_table: &mut IdentifierTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            ForInit::ExpressionInit(Some(expr_node)) => Ok(expr_node.resolve(identifier_table, label_map)?),
            ForInit::ExpressionInit(None) => Ok(()),
            ForInit::DeclarationInit(declaration_node) => Ok(declaration_node.resolve(identifier_table, label_map)?)
        }
    }
}

impl CheckGotoLabelBreakContinue for StatementNode {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels, case_map: &mut Option<&mut HashMap<i32, u32>>, default_label: &mut Option<u32>) -> Result<(), CompilerErrors> {
        match self {
            StatementNode::IfStmt { condition: _, stmt, else_stmt } =>  {
                stmt.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label)?;
                if let Some(else_stmt_unwrapped) = else_stmt {
                    else_stmt_unwrapped.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label)?;
                }
                Ok(())
            },
            StatementNode::WhileStmt { condition: _, stmt, loop_labels } => {
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let mut labelgen_singleton = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap();
                if let None = loop_labels.break_label() {
                    loop_labels.set_break_label(Some(labelgen_singleton.gen()));
                }
                if let None = loop_labels.continue_label() {
                    loop_labels.set_continue_label(Some(labelgen_singleton.gen()));
                }
                drop(labelgen_singleton);
                Ok(stmt.check_goto_label_break_continue(true, is_inside_switch, label_map, loop_labels, case_map, default_label)?)
            },
            StatementNode::DoWhileStmt { condition: _, stmt, loop_labels } => {
                stmt.check_goto_label_break_continue(true, is_inside_switch, label_map, loop_labels, case_map, default_label)?;
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let mut labelgen_singleton = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap();
                if let None = loop_labels.break_label() {
                    loop_labels.set_break_label(Some(labelgen_singleton.gen()));
                }
                Ok(())
            },
            StatementNode::ForStmt { init: _ , condition: _, post_expr: _, stmt, loop_labels } => {
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let mut labelgen_singleton = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap();
                if let None = loop_labels.break_label() {
                    loop_labels.set_break_label(Some(labelgen_singleton.gen()));
                }
                if let None = loop_labels.continue_label() {
                    loop_labels.set_continue_label(Some(labelgen_singleton.gen()));
                }
                drop(labelgen_singleton);
                Ok(stmt.check_goto_label_break_continue(true, is_inside_switch, label_map, loop_labels, case_map, default_label)?)
            },
            StatementNode::SwitchStmt { condition: _, stmt, loop_labels, cases_map, default_stmt_label } => {
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                loop_labels.set_break_label(Some(LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen()));
                Ok(stmt.check_goto_label_break_continue(is_inside_loop, true, label_map, loop_labels, &mut Some(cases_map), default_stmt_label)?)
            },
            StatementNode::CaseStmt { label, value, stmt } =>  {
                if !is_inside_switch {
                    eprintln!("Error: case stmt cannot be outside switch");
                    return Err(CompilerErrors::SemanticError)
                }
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                *label = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen();
                if let Some(switch_hashmap) = case_map {
                    if let ExprNode::Constant(case_value) = value {
                        switch_hashmap.insert(*case_value, *label);
                    }
                }
                Ok(stmt.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label)?)
            },
            StatementNode::DefaultStmt { label, stmt} => {
                if !is_inside_switch {
                    eprintln!("Error: default stmt cannot be outside switch");
                    return Err(CompilerErrors::SemanticError)
                }
                if let Some(_) = default_label {
                    eprintln!("Error: in a switch statement cannot be more than one 'default' statement");
                    return Err(CompilerErrors::SemanticError)
                }
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                *label = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen();
                *default_label = Some(*label);
                Ok(stmt.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label)?)
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
            StatementNode::BreakStmt(break_label_stmt) => {
                if !is_inside_loop && !is_inside_switch {
                    eprintln!("Error: break stmt cannot be outside switch or loops");
                    return Err(CompilerErrors::SemanticError)
                }
                if let Some(break_label) = loop_labels.break_label() {
                    *break_label_stmt = Some(break_label);
                } else {
                    LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                    *break_label_stmt = Some(LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen());
                }
                loop_labels.set_break_label(break_label_stmt.clone());
                Ok(())
            },
            StatementNode::ContinueStmt(continue_label_stmt) => {
                if !is_inside_loop {
                    eprintln!("Error: continue stmt must be inside a loop");
                    return Err(CompilerErrors::SemanticError)
                }
                if let Some(continue_label) = loop_labels.continue_label() {
                    *continue_label_stmt = Some(continue_label);
                } else {
                    LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                    *continue_label_stmt = Some(LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen());
                }
                loop_labels.set_continue_label(continue_label_stmt.clone());
                Ok(())
            },
            StatementNode::LabelStmt { label_name: _, label_name_index: _, stmt} => stmt.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label),
            StatementNode::Expr(_) => {
                // Nothing to do
                Ok(())
            },
            StatementNode::Compound(block_node) => block_node.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label),
            StatementNode::EmptyStmt => {
                // Note: nothing to do
                Ok(())
            }
        }
    }
}