use std::sync::Mutex;
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::tacky::label_gen::{LabelGen, LABEL_GEN_SINGLETON};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;

#[derive(Debug)]
pub enum StatementNode {
    IfStmt {
        condition: ExprNode,
        stmt: Box<StatementNode>,
        else_stmt: Option<Box<StatementNode>>
    },
    ReturnStmt(ExprNode),
    Goto {
        label_name: String,
        label_name_index: u32
    },
    LabelStmt {
        label_name: String,
        label_name_index: u32,
        stmt: Box<StatementNode>,
    },
    Expr(ExprNode),
    EmptyStmt,
}

impl GenerateTackyInstructions<()> for StatementNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        match self {
            StatementNode::IfStmt { condition, stmt, else_stmt} => {
                let condition_tacky = condition.to_tacky(tacky_instructions);
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let label_if_false = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen();
                tacky_instructions.push(InstructionTackyNode::JmpIfZero { condition: condition_tacky, jmp_label_target: label_if_false });
                stmt.to_tacky(tacky_instructions);
                if let Some(else_stmt_unwrapped) = else_stmt {
                    let label_after_else = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen();
                    tacky_instructions.push(InstructionTackyNode::Jmp(label_after_else));
                    tacky_instructions.push(InstructionTackyNode::Label(label_if_false));
                    else_stmt_unwrapped.to_tacky(tacky_instructions);
                    tacky_instructions.push(InstructionTackyNode::Label(label_after_else));
                } else {
                    tacky_instructions.push(InstructionTackyNode::Label(label_if_false));
                }
            },
            StatementNode::ReturnStmt(expr) => {
                let expr_tacky = expr.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::Return(expr_tacky))
            },
            StatementNode::Goto { label_name: _, label_name_index } => tacky_instructions.push(InstructionTackyNode::Jmp(*label_name_index)),
            StatementNode::LabelStmt { label_name: _, stmt, label_name_index } => {
                tacky_instructions.push(InstructionTackyNode::Label(*label_name_index));
                stmt.to_tacky(tacky_instructions)
            },
            StatementNode::Expr(expr) =>  {
                expr.to_tacky(tacky_instructions);
            },
            StatementNode::EmptyStmt => {
                // Note: nothing to do
            }
        }
    }
}

impl AstDebugPrinter for StatementNode {
    fn debug_visit(&self) {
        match self {
            StatementNode::IfStmt { condition, stmt, else_stmt} => {
                println!("IfStmt(");
                println!("condition= ");
                condition.debug_visit();
                println!("stmt= ");
                stmt.debug_visit();
                println!("else = ");
                else_stmt.as_ref().map(|v| v.debug_visit());
                println!(")");
            },
            StatementNode::ReturnStmt(expr) => {
                println!("Return(");
                expr.debug_visit();
                println!(")");
            },
            StatementNode::Goto { label_name, label_name_index } => println!("Goto(label_name: {}, label_name_index: {})", label_name, label_name_index),
            StatementNode::LabelStmt { label_name, label_name_index, stmt} => {
                println!("Label(label_name: {}, label_name_index: {}", label_name, label_name_index);
                println!("stmt: ");
                stmt.debug_visit();
                println!(")");
            },
            StatementNode::Expr(expr) => expr.debug_visit(),
            StatementNode::EmptyStmt => println!("EmptyStmt")
        }
    }
}
