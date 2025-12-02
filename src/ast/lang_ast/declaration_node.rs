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

use crate::ast::lang_ast::expr_node::ExprNode;
use crate::ast::lang_ast::function_declaration_node::FunctionDeclarationNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_val_node::{TemporaryVar, ValTackyNode};

#[derive(Debug)]
pub enum DeclarationNode {
    VariableDeclaration {
        var_name: String,
        init: Option<ExprNode>
    },
    FunctionDeclaration(FunctionDeclarationNode),
}

impl GenerateTackyInstructions<()> for DeclarationNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        // Note: if the declaration is without an initializzation expression, there nothing to do in tacky generation
        if let DeclarationNode::VariableDeclaration { var_name: _, init: Some(init_expr) } = self {
            let init_expr_tacky = init_expr.to_tacky(tacky_instructions);
            tacky_instructions.push(InstructionTackyNode::Copy { src: init_expr_tacky, dest: ValTackyNode::Var(TemporaryVar::generate()) });
        }
    }
}

impl AstDebugPrinter for DeclarationNode {
    fn debug_visit(&self) {
        match self {
            DeclarationNode::VariableDeclaration { var_name, init } => {
                println!("VariableDeclaration(");
                print!("var_name: {}, init: ", var_name);
                if let Some(expr) = init {
                    expr.debug_visit();
                } else {
                    println!("None");
                }
                println!(")");
            },
            DeclarationNode::FunctionDeclaration(func_decl_node) => {
                func_decl_node.debug_visit();
            }
        }
    }
}