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

pub mod program_parse;
pub mod statement_parse; 
pub mod factor_parse;
pub mod unop_parse;
pub mod expr_parse;
pub mod binop_parse;
pub mod block_item_parse;
pub mod declaration_parse;
pub mod block_parse;
pub mod for_init_parse;
pub mod function_declaration_parse;
pub mod var_declaration_parse;