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
use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::ast::lang_ast::block_node::BlockNode;
use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::ast::lang_ast::function_declaration_node::FunctionDeclarationNode;
use crate::ast::lang_ast::statement_node::LoopLabels;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_break_continue_trait::CheckGotoLabelBreakContinue;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::identifier_table::IdentifierTable;

impl ResolveVarExprLabel for FunctionDeclarationNode {
    fn resolve(&mut self, identifier_table: &mut IdentifierTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let FunctionDeclarationNode::FunctionDef { func_name, params, block_option } = self;
        identifier_table.insert_ext_link_identifier(func_name.to_string())?;
        if let Some(block) = block_option {
            let BlockNode::Item(block_item_node) = block;
            //Note: function variable params must be in the same function block
            for param in params.iter().rev() {
                block_item_node.push_front(BlockItemNode::Declaration(DeclarationNode::VariableDeclaration { var_name: param.to_string(), var_name_index: 0, init: None }))
            }
            block.resolve(identifier_table, label_map)?;
        }
        //Note: in case of function without a body, for example during a function prototype declaration there is nothing to do
        Ok(())
    }
}

impl CheckGotoLabelBreakContinue for FunctionDeclarationNode {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels, case_list: &mut Option<&mut HashMap<i32, u32>>, default_label: &mut Option<u32>) -> Result<(), CompilerErrors> {
        let FunctionDeclarationNode::FunctionDef { func_name: _, params: _,  block_option } = self;
        if let Some(block) = block_option {
            block.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_list, default_label)?;
        }
        //Note: in case of function without a body, for example during a function prototype declaration there is nothing to do
        Ok(())
    }
}