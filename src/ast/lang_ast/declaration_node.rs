use crate::ast::lang_ast::expr_node::ExprNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_val_node::ValTackyNode;

#[derive(Debug)]
pub enum DeclarationNode {
    VariableDeclaration {
        var_name: String,
        var_name_index: u32,
        init: Option<ExprNode>
    }
}

impl GenerateTackyInstructions<()> for DeclarationNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        // Note: if the declaration is without an initializzation expression, there nothing to do in tacky generation
        if let DeclarationNode::VariableDeclaration { var_name: _, var_name_index, init: Some(init_expr) } = self {
            let init_expr_tacky = init_expr.to_tacky(tacky_instructions);
            tacky_instructions.push(InstructionTackyNode::Copy { src: init_expr_tacky, dest: ValTackyNode::Var(*var_name_index)});
        }
    }
}

impl AstDebugPrinter for DeclarationNode {
    fn debug_visit(&self) {
        let DeclarationNode::VariableDeclaration { var_name, var_name_index, init } = self;
        println!("VariableDeclaration(");
        print!("var_name: {}, var_index: {}, init: ", var_name, var_name_index);
        if let Some(expr) = init {
            expr.debug_visit();
        } else {
            println!("None");
        }
        println!(")");
    }
}