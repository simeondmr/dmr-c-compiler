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
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::identifier_table::IdentifierTable;

impl ResolveVarExprLabel for ExprNode {
    fn resolve(&mut self, identifier_table: &mut IdentifierTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        match self {
            ExprNode::Constant(_) => Ok(()),
            ExprNode::Unary { unary_operator: _, expr } => expr.resolve(identifier_table, label_map),
            ExprNode::Binary { binary_operator: _, left_expr, right_expr } => {
                left_expr.resolve(identifier_table, label_map)?;
                right_expr.resolve(identifier_table, label_map)
            },
            ExprNode::Assignment { assignment_type: _, dest, expr } => {
                if !matches!(&**dest, ExprNode::Var { .. }) {
                    eprintln!("Error: expected var in lvalue");
                    return Err(CompilerErrors::SemanticError);
                }
                dest.resolve(identifier_table, label_map)?;
                expr.resolve(identifier_table, label_map)
            },
            ExprNode::PrePostOperator { pre_post_operator_type: _, identifier} => identifier.resolve(identifier_table, label_map),
            ExprNode::Conditional { condition, true_expr, false_expr } => {
                condition.resolve(identifier_table, label_map)?;
                true_expr.resolve(identifier_table, label_map)?;
                false_expr.resolve(identifier_table, label_map)
            },
            ExprNode::Var { var_name, var_name_index } => {
                if let Some(new_var_name_index) = identifier_table.var_index(var_name) {
                    *var_name_index = new_var_name_index;
                } else {
                    eprintln!("Error: undeclared var {}", var_name);
                    return Err(CompilerErrors::SemanticError);
                }
                Ok(())
            },
            ExprNode::FunctionCall { name, args } => {
                let _ = identifier_table.ext_identifier(name.to_string())?;
                for arg in args {
                    arg.resolve(identifier_table, label_map)?;
                }
                Ok(())
            }
        }
    }
}