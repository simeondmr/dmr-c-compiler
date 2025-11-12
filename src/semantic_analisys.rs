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

pub mod resolve_var_expr_trait;
pub mod program_node_semantic_analisys;
pub mod block_item_node_semantic_analisys;
pub mod declaration_node_semantic_analisys;
pub mod expr_node_semantic_analisys;
pub mod statement_node_semantic_analisys;
pub mod semantic_analisys_core;
pub mod check_goto_label_break_continue_trait;
pub mod block_node_semantic_analisys;
pub mod identifier_table;
pub mod function_declaration_node_semantic_analisys;
pub mod type_check_semantic_analisys_trait;