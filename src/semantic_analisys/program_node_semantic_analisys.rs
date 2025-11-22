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
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::ast::lang_ast::statement_node::LoopLabels;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::check_goto_label_break_continue_trait::{ CheckGotoLabelBreakContinue };
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::identifier_table::IdentifierTable;
use crate::semantic_analisys::type_check_semantic_analisys_trait::TypeCheck;
use crate::symbol_table::symbol_table::SymbolTable;

impl ResolveVarExprLabel for ProgramNode {
    fn resolve(&mut self, identifier_table: &mut IdentifierTable, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let ProgramNode::ProgramDef(functions) = self;
        for function in functions {
            function.resolve(identifier_table, label_map)?;
        }
        Ok(())
    }
}

impl CheckGotoLabelBreakContinue for ProgramNode {
    fn check_goto_label_break_continue(&mut self, is_inside_loop: bool, is_inside_switch: bool, label_map: &mut HashMap<String, u32>, loop_labels: &mut LoopLabels, case_map: &mut Option<&mut HashMap<i32, u32>>, default_label: &mut Option<u32>) -> Result<(), CompilerErrors> {
        let ProgramNode::ProgramDef(functions) = self;
        for function in functions {
            function.check_goto_label_break_continue(is_inside_loop, is_inside_switch, label_map, loop_labels, case_map, default_label)?;
        }
        Ok(())
    }
}

impl TypeCheck for ProgramNode {
    fn type_check(&mut self, symbol_table: &mut SymbolTable, is_inside_function: bool) -> Result<(), CompilerErrors> {
        let ProgramNode::ProgramDef(functions) = self;
        for function in functions {
            function.type_check(symbol_table, is_inside_function)?;
        }
        Ok(())
    }
}