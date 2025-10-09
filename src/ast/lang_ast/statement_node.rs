use std::collections::HashMap;
use std::sync::Mutex;
use crate::ast::lang_ast::block_node::BlockNode;
use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTackyInstructions};
use crate::tacky::binary_operator_tacky_node::BinaryOperatorTackyNode;
use crate::tacky::label_gen::{LabelGen, LABEL_GEN_SINGLETON};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_val_node::{TemporaryVar, ValTackyNode};

#[allow(dead_code)]
#[derive(Debug)]
pub enum StatementNode {
    IfStmt {
        condition: ExprNode,
        stmt: Box<StatementNode>,
        else_stmt: Option<Box<StatementNode>>
    },
    WhileStmt {
        condition: ExprNode,
        stmt: Box<StatementNode>,
        loop_labels: LoopLabels
    },
    DoWhileStmt {
        condition: ExprNode,
        stmt: Box<StatementNode>,
        loop_labels: LoopLabels
    },
    ForStmt {
        init: ForInit,
        condition: Option<ExprNode>,
        post_expr: Option<ExprNode>,
        stmt: Box<StatementNode>,
        loop_labels: LoopLabels
    },
    SwitchStmt {
        condition: ExprNode,
        stmt: Box<StatementNode>,
        loop_labels: LoopLabels,
        cases_map: HashMap<i32, u32>,
        default_stmt_label: Option<u32>
    },
    CaseStmt {
        label: u32,
        value: ExprNode,
        stmt: Box<StatementNode>
    },
    DefaultStmt {
        label: u32,
        stmt: Box<StatementNode>
    },
    ReturnStmt(ExprNode),
    Goto {
        label_name: String,
        label_name_index: u32
    },
    BreakStmt(Option<u32>),
    ContinueStmt(Option<u32>),
    LabelStmt {
        label_name: String,
        label_name_index: u32,
        stmt: Box<StatementNode>,
    },
    Compound(BlockNode),
    Expr(ExprNode),
    EmptyStmt,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum ForInit {
    ExpressionInit(Option<ExprNode>),
    DeclarationInit(DeclarationNode)
}

#[derive(Debug)]
pub struct LoopLabels {
    break_label: Option<u32>,
    continue_label: Option<u32>
}

impl LoopLabels {
    pub fn new() -> LoopLabels {
        LoopLabels {
            break_label: None,
            continue_label: None,
        }
    }

    pub fn break_label(&self) -> Option<u32> {
        self.break_label
    }

    pub fn continue_label(&self) -> Option<u32> {
        self.continue_label
    }

    pub fn set_break_label(&mut self, break_label: Option<u32>) {
        self.break_label = break_label
    }

    pub fn set_continue_label(&mut self, continue_label: Option<u32>) {
        self.continue_label = continue_label
    }
}

impl<'a> GenerateTackyInstructions<()> for StatementNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        match self {
            StatementNode::IfStmt { condition, stmt, else_stmt} => {
                let condition_tacky_node = condition.to_tacky(tacky_instructions);
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let label_if_false = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen();
                tacky_instructions.push(InstructionTackyNode::JmpIfZero { condition: condition_tacky_node, jmp_label_target: label_if_false });
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
            StatementNode::WhileStmt { condition, stmt, loop_labels } => {
                tacky_instructions.push(InstructionTackyNode::Label(loop_labels.continue_label.unwrap()));
                let condition_tacky_node = condition.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::JmpIfZero { condition: condition_tacky_node, jmp_label_target: loop_labels.break_label.unwrap() });
                stmt.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::Jmp(loop_labels.continue_label.unwrap()));
                tacky_instructions.push(InstructionTackyNode::Label(loop_labels.break_label.unwrap()));
            },
            StatementNode::DoWhileStmt { condition, stmt, loop_labels} => {
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let start_loop_label = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen();
                tacky_instructions.push(InstructionTackyNode::Label(start_loop_label));
                stmt.to_tacky(tacky_instructions);
                if let Some(continue_label) = loop_labels.continue_label {
                    tacky_instructions.push(InstructionTackyNode::Label(continue_label));
                }
                let condition_tacky_node = condition.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::JmpIfNotZero { condition: condition_tacky_node, jmp_label_target: start_loop_label });
                tacky_instructions.push(InstructionTackyNode::Label(loop_labels.break_label.unwrap()));
            },
            StatementNode::ForStmt { init, condition, post_expr, stmt, loop_labels } => {
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let start_loop_label = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap().gen();
                init.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::Label(start_loop_label));
                if let Some(condition_node) = condition {
                    let condition_tacky_node = condition_node.to_tacky(tacky_instructions);
                    tacky_instructions.push(InstructionTackyNode::JmpIfZero { condition: condition_tacky_node, jmp_label_target: loop_labels.break_label.unwrap() });
                }
                stmt.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::Label(loop_labels.continue_label.unwrap()));
                if let Some(post_expr_node) = post_expr {
                    post_expr_node.to_tacky(tacky_instructions);
                }
                tacky_instructions.push(InstructionTackyNode::Jmp(start_loop_label));
                tacky_instructions.push(InstructionTackyNode::Label(loop_labels.break_label.unwrap()));
            },
            StatementNode::SwitchStmt { condition, stmt, loop_labels, cases_map, default_stmt_label } => {
                let condition = condition.to_tacky(tacky_instructions);
                cases_map.iter().for_each(|(case_value, case_label)| {
                    let case_cmp_tmp_var = TemporaryVar::generate();
                    tacky_instructions.push(InstructionTackyNode::Binary {
                        binary_operator: BinaryOperatorTackyNode::Equal,
                        left_expr: condition.clone(),
                        right_expr: ValTackyNode::Constant(*case_value),
                        dest: ValTackyNode::Var(case_cmp_tmp_var)
                    });
                    tacky_instructions.push(InstructionTackyNode::JmpIfNotZero { condition: ValTackyNode::Var(case_cmp_tmp_var), jmp_label_target: *case_label });
                });
                default_stmt_label.map(|default_label_value| tacky_instructions.push(InstructionTackyNode::Jmp(default_label_value)));
                stmt.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::Label(loop_labels.break_label.unwrap()));
            },
            StatementNode::CaseStmt { label, value: _, stmt } | StatementNode::DefaultStmt { label, stmt } => {
                tacky_instructions.push(InstructionTackyNode::Label(*label));
                stmt.to_tacky(tacky_instructions);
            },
            StatementNode::ReturnStmt(expr) => {
                let expr_tacky = expr.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::Return(expr_tacky))
            },
            StatementNode::Goto { label_name: _, label_name_index } => tacky_instructions.push(InstructionTackyNode::Jmp(*label_name_index)),
            StatementNode::BreakStmt(break_label) => {
                if let Some(break_label) = break_label {
                    tacky_instructions.push(InstructionTackyNode::Jmp(*break_label));
                }
            },
            StatementNode::ContinueStmt(continue_label) => {
                if let Some(continue_label) = continue_label {
                    tacky_instructions.push(InstructionTackyNode::Jmp(*continue_label));
                }
            },
            StatementNode::LabelStmt { label_name: _, stmt, label_name_index } => {
                tacky_instructions.push(InstructionTackyNode::Label(*label_name_index));
                stmt.to_tacky(tacky_instructions)
            },
            StatementNode::Compound(block_node) => block_node.to_tacky(tacky_instructions),
            StatementNode::Expr(expr) =>  {
                expr.to_tacky(tacky_instructions);
            },
            StatementNode::EmptyStmt => {
                // Note: nothing to do
            }
        }
    }
}

impl GenerateTackyInstructions<()> for ForInit {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> () {
        if let ForInit::DeclarationInit(declaration_node) = self {
            declaration_node.to_tacky(tacky_instructions);
        } else if let ForInit::ExpressionInit(Some(expr_node)) = self {
            expr_node.to_tacky(tacky_instructions);
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
                println!("stmt: ");
                stmt.debug_visit();
                println!("else: ");
                else_stmt.as_ref().map(|v| v.debug_visit());
                println!(")");
            },
            StatementNode::WhileStmt { condition, stmt, loop_labels } => {
                println!("WhileStmt(");
                print!("condition:");
                condition.debug_visit();
                print!("stmt: ");
                stmt.debug_visit();
                println!("\tbreak_label: {:?}", loop_labels.break_label);
                println!("\tcontinue_label: {:?}", loop_labels.continue_label);
                println!(")");
            },
            StatementNode::DoWhileStmt { condition, stmt, loop_labels } => {
                println!("DoWhileStmt(");
                print!("condition:");
                condition.debug_visit();
                print!("stmt: ");
                stmt.debug_visit();
                println!("\tbreak_label: {:?}", loop_labels.break_label);
                println!("\tcontinue_label: {:?}", loop_labels.continue_label);
                println!(")");
            },
            StatementNode::ForStmt { init: _, condition: _, post_expr: _,  stmt: _, loop_labels: _ } => {

            },
            StatementNode::SwitchStmt { condition, stmt, loop_labels, cases_map,default_stmt_label} => {
                println!("Switch(");
                println!("Condition:");
                condition.debug_visit();
                println!("cases list:");
                cases_map.iter().for_each(|(case_value,case_label) | println!("(case constant value: {}, case label: {})", case_value, case_label));
                println!("Stmt: ");
                stmt.debug_visit();
                println!("break_label: {:?}", loop_labels);
                println!("default_stmt_label {:?}", default_stmt_label);
                println!(")");
            },
            StatementNode::CaseStmt { label, value, stmt } => {
                println!("Case(");
                println!("label: {}", label);
                value.debug_visit();
                stmt.debug_visit();
                println!(")");
            },
            StatementNode::DefaultStmt { label, stmt } => {
                println!("Default(");
                println!("label: {}", label);
                println!("stmt:");
                stmt.debug_visit();
                println!(")");
            },
            StatementNode::ReturnStmt(expr) => {
                println!("Return(");
                expr.debug_visit();
                println!(")");
            },
            StatementNode::Goto { label_name, label_name_index } => println!("Goto(label_name: {}, label_name_index: {})", label_name, label_name_index),
            StatementNode::BreakStmt(break_label) => println!("break(break_label_index: {:?})", break_label),
            StatementNode::ContinueStmt(continue_label) => println!("Continue(continue_label_index: {:?})", continue_label),
            StatementNode::LabelStmt { label_name, label_name_index, stmt} => {
                println!("Label(label_name: {}, label_name_index: {}", label_name, label_name_index);
                println!("stmt: ");
                stmt.debug_visit();
                println!(")");
            },
            StatementNode::Compound(block_node) => {
                println!("Compound(");
                block_node.debug_visit();
                println!(")");
            },
            StatementNode::Expr(expr) => expr.debug_visit(),
            StatementNode::EmptyStmt => println!("EmptyStmt")
        }
    }
}
