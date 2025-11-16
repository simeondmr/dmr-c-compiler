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

use crate::ast::lang_ast::function_declaration_node::FunctionDeclarationNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky};
use crate::tacky::tacky_program_node::ProgramTackyNode;

#[derive(Debug)]
pub enum ProgramNode {
    ProgramDef(Vec<FunctionDeclarationNode>)
}

impl GenerateTacky<ProgramTackyNode> for ProgramNode {
    fn to_tacky(&self) -> ProgramTackyNode {
        let ProgramNode::ProgramDef(functions) = self;
        let mut functions_tacky = Vec::new();
        functions.iter().for_each(|function| {
            //Note: in order to avoid to create a FunctionTackyNode also for function prototype
            let FunctionDeclarationNode::FunctionDef { func_name: _, params: _, block_option } = function;
            if block_option.is_some() {
                functions_tacky.push(function.to_tacky())
            }
        });
        ProgramTackyNode::ProgramDef(functions_tacky)
    }
}

impl AstDebugPrinter for ProgramNode {
    fn debug_visit(&self) {
        let ProgramNode::ProgramDef(functions) = self;
        println!("Program(");
        functions.iter().for_each(|function| function.debug_visit());
        println!(")");
    }
}