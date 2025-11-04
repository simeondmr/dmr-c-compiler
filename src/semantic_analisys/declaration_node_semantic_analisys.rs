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
use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::identifier_table::IdentifierTable;
use crate::tacky::tacky_val_node::TemporaryVar;

impl ResolveVarExprLabel for DeclarationNode {
    fn resolve(&mut self, identifier_table: &mut IdentifierTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        if let DeclarationNode::VariableDeclaration { var_name, var_name_index, init } = self {
            let new_var_name_index = TemporaryVar::generate();
            identifier_table.new_local_variable(var_name.to_string(), new_var_name_index)?;
            *var_name_index = new_var_name_index;
            if let Some(expr) = init {
                return expr.resolve(identifier_table, label_map);
            }
        } else if let DeclarationNode::FunctionDeclaration(function_declaration_node) = self {
            function_declaration_node.resolve(identifier_table, label_map)?;
        }
        Ok(())
    }
}