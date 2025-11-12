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

use crate::ast::lang_ast::block_node::BlockNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky, GenerateTackyInstructions};
use crate::tacky::tacky_function_node::FunctionTackyNode;

#[derive(Debug)]
pub enum FunctionDeclarationNode {
    FunctionDef {
        func_name: String,
        params: Vec<String>,
        block_option: Option<BlockNode>
    }
}

impl GenerateTacky<FunctionTackyNode> for FunctionDeclarationNode {
    fn to_tacky(&self) -> FunctionTackyNode {
        let FunctionDeclarationNode::FunctionDef { func_name, params, block_option } = self;
        let mut tacky_instructions = Vec::new();
        if let Some(block) = block_option {
            block.to_tacky(&mut tacky_instructions);
        }
        FunctionTackyNode::FunctionDef { func_name: func_name.to_string(), params: params.to_vec(), tacky_instructions }
    }
}

impl AstDebugPrinter for FunctionDeclarationNode {
    fn debug_visit(&self) {
        println!("{:?}", self);
    }
}