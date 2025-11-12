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
use crate::semantic_analisys::check_goto_label_break_continue_trait::CheckGotoLabelBreakContinue;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExprLabel;
use crate::semantic_analisys::identifier_table::IdentifierTable;
use crate::semantic_analisys::type_check_semantic_analisys_trait::TypeCheck;
use crate::symbol_table::symbol_table::SymbolTable;

pub fn semantic_analisys(program_node: &mut ProgramNode) -> Result<(), CompilerErrors> {
    let mut identifier_table = IdentifierTable::new();
    let mut label_map: HashMap<String, u32> = HashMap::new();
    let mut symbol_table = SymbolTable::new();
    program_node.resolve(&mut identifier_table, &mut label_map)?;
    program_node.check_goto_label_break_continue(false, false, &mut label_map, &mut LoopLabels::new(), &mut None, &mut None)?;
    program_node.type_check(&mut symbol_table, false)
}