use std::sync::Mutex;
use crate::ast::lang_ast::binary_operator_node::BinaryOperatorNode;
use crate::ast::lang_ast::lang_ast_visit_trait::{AstDebugPrinter, GenerateTacky, GenerateTackyInstructions};
use crate::ast::lang_ast::unary_operator_node::UnaryOperatorNode;
use crate::tacky::binary_operator_tacky_node::BinaryOperatorTackyNode;
use crate::tacky::label_gen::{LabelGen, LABEL_GEN_SINGLETON};
use crate::tacky::tacky_instruction_node::InstructionTackyNode;
use crate::tacky::tacky_val_node::{TemporaryVar, ValTackyNode};

#[derive(Debug)]
pub enum ExprNode {
    Constant(i32),
    Unary {
        unary_operator: UnaryOperatorNode,
        expr: Box<ExprNode>
    },
    Binary {
        binary_operator: BinaryOperatorNode,
        left_expr: Box<ExprNode>,
        right_expr: Box<ExprNode>
    },
    Assignment {
        assignment_type: AssignmentOperatorType,
        dest: Box<ExprNode>, 
        expr: Box<ExprNode>
    },
    Var {
        var_name: String,
        var_name_index: u32
    },
}

#[derive(Debug)]
pub enum AssignmentOperatorType {
    AssignmentDefault,
    AssignmentAdd,
    AssignmentSub,
    AssignmentMultiply,
    AssignmentDivide,
    AssignmentReminder,
    AssignmentBitwiseOr,
    AssignmentBitwiseAnd,
    AssignmentBitwiseXor,
    AssignmentBitwiseLeftShift,
    AssignmentBitwiseRightShift,
}

impl GenerateTackyInstructions<ValTackyNode> for ExprNode {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> ValTackyNode {
        match self {
            ExprNode::Constant(value) => ValTackyNode::Constant(*value),
            ExprNode::Unary { unary_operator , expr } => {
                let src = expr.to_tacky(tacky_instructions);
                let dest = ValTackyNode::Var(TemporaryVar::generate());
                tacky_instructions.push(InstructionTackyNode::Unary { unary_operator: unary_operator.to_tacky(), src, dest: dest.clone() });
                dest
            },
            ExprNode::Binary { binary_operator , left_expr, right_expr}  if matches!(binary_operator, BinaryOperatorNode::And) => {
                let left_exp_tacky_node = left_expr.to_tacky(tacky_instructions);
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let mut labelgen_singleton = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap();
                let label_false_case = labelgen_singleton.gen();
                let end_label = labelgen_singleton.gen();
                tacky_instructions.push(InstructionTackyNode::JmpIfZero { condition: left_exp_tacky_node, jmp_label_target: label_false_case });
                let right_exp_tacky_node = right_expr.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::JmpIfZero { condition: right_exp_tacky_node, jmp_label_target: label_false_case });
                let result = ValTackyNode::Var(TemporaryVar::generate());
                tacky_instructions.push(InstructionTackyNode::Copy { src: ValTackyNode::Constant(1), dest: result.clone() });
                tacky_instructions.push(InstructionTackyNode::Jmp(end_label));
                tacky_instructions.push(InstructionTackyNode::Label(label_false_case));
                tacky_instructions.push(InstructionTackyNode::Copy { src: ValTackyNode::Constant(0), dest: result.clone() });
                tacky_instructions.push(InstructionTackyNode::Label(end_label));
                result
            },
            ExprNode::Binary { binary_operator , left_expr, right_expr}  if matches!(binary_operator, BinaryOperatorNode::Or) => {
                let left_exp_tacky_node = left_expr.to_tacky(tacky_instructions);
                LABEL_GEN_SINGLETON.get_or_init(|| Mutex::new(LabelGen::new()));
                let mut labelgen_singleton = LABEL_GEN_SINGLETON.get().unwrap().lock().unwrap();
                let label_true_case = labelgen_singleton.gen();
                let end_label = labelgen_singleton.gen();
                tacky_instructions.push(InstructionTackyNode::JmpIfNotZero { condition: left_exp_tacky_node, jmp_label_target: label_true_case });
                let right_exp_tacky_node = right_expr.to_tacky(tacky_instructions);
                tacky_instructions.push(InstructionTackyNode::JmpIfNotZero { condition: right_exp_tacky_node, jmp_label_target: label_true_case });
                let result = ValTackyNode::Var(TemporaryVar::generate());
                tacky_instructions.push(InstructionTackyNode::Copy { src: ValTackyNode::Constant(0), dest: result.clone() });
                tacky_instructions.push(InstructionTackyNode::Jmp(end_label));
                tacky_instructions.push(InstructionTackyNode::Label(label_true_case));
                tacky_instructions.push(InstructionTackyNode::Copy { src: ValTackyNode::Constant(1), dest: result.clone() });
                tacky_instructions.push(InstructionTackyNode::Label(end_label));
                result
            },
            ExprNode::Binary { binary_operator, left_expr, right_expr } => {
                let left_exp_tacky_node = left_expr.to_tacky(tacky_instructions);
                let right_exp_tacky_node = right_expr.to_tacky(tacky_instructions);
                let dest = ValTackyNode::Var(TemporaryVar::generate());
                tacky_instructions.push(InstructionTackyNode::Binary { binary_operator: binary_operator.to_tacky(), left_expr: left_exp_tacky_node, right_expr: right_exp_tacky_node, dest: dest.clone() });
                dest
            },
            ExprNode::Var { var_name: _, var_name_index} => {
                // Note: after the semantic analisys the var_name_index is fixed
                ValTackyNode::Var(*var_name_index)
            },
            ExprNode::Assignment { assignment_type, dest, expr } if matches!(&**dest, ExprNode::Var { var_name: _, var_name_index: _ }) => {
                let right_tacky_node = expr.to_tacky(tacky_instructions);
                let mut var_index = 0;
                if let ExprNode::Var { var_name: _, var_name_index } = &**dest {
                    var_index = *var_name_index;
                }
                if let AssignmentOperatorType::AssignmentDefault = assignment_type {
                    tacky_instructions.push(InstructionTackyNode::Copy { src: right_tacky_node, dest: ValTackyNode::Var(var_index) });
                } else {
                    tacky_instructions.push(InstructionTackyNode::Binary { binary_operator: assignment_type.to_tacky(), left_expr: ValTackyNode::Var(var_index), right_expr: right_tacky_node.clone(), dest: ValTackyNode::Var(var_index) });
                }
                ValTackyNode::Var(var_index)
            },
            _ => {
                //Note: for covering ExprNode::Assignment(left_expr, right_expr) where left_expr is not a Var...note that here this case can't happen because of semantic analisys
                ValTackyNode::Empty
            }
        }
    }
}

impl GenerateTacky<BinaryOperatorTackyNode> for AssignmentOperatorType {
    fn to_tacky(&self) -> BinaryOperatorTackyNode {
        match self {
            AssignmentOperatorType::AssignmentDefault => BinaryOperatorTackyNode::Empty,
            AssignmentOperatorType::AssignmentAdd => BinaryOperatorTackyNode::Add,
            AssignmentOperatorType::AssignmentSub => BinaryOperatorTackyNode::Subtract,
            AssignmentOperatorType::AssignmentMultiply => BinaryOperatorTackyNode::Multiply,
            AssignmentOperatorType::AssignmentDivide => BinaryOperatorTackyNode::Divide,
            AssignmentOperatorType::AssignmentReminder => BinaryOperatorTackyNode::Remainder,
            AssignmentOperatorType::AssignmentBitwiseOr => BinaryOperatorTackyNode::BitwiseOr,
            AssignmentOperatorType::AssignmentBitwiseAnd => BinaryOperatorTackyNode::BitwiseAnd,
            AssignmentOperatorType::AssignmentBitwiseXor => BinaryOperatorTackyNode::BitwiseXor,
            AssignmentOperatorType::AssignmentBitwiseLeftShift => BinaryOperatorTackyNode::BitwiseLeftShift,
            AssignmentOperatorType::AssignmentBitwiseRightShift => BinaryOperatorTackyNode::BitwiseRightShift
        }
    }
}

impl AstDebugPrinter for ExprNode {
    fn debug_visit(&self) {
        match self {
            ExprNode::Constant(value) => println!("Constant({})", value),
            ExprNode::Unary { unary_operator, expr } => {
                println!("Unary {:?} (", unary_operator);
                expr.debug_visit();
                println!(")");
            },
            ExprNode::Binary { binary_operator, left_expr, right_expr } => {
                println!("Binary {:?} (", binary_operator);
                left_expr.debug_visit();
                right_expr.debug_visit();
                println!(")");
            },
            ExprNode::Assignment { assignment_type, dest, expr } => {
                print!("Assignment(assignment_type: ");
                assignment_type.debug_visit();
                print!("dest: ");
                dest.debug_visit();
                print!("expr: ");
                expr.debug_visit();
                println!(")");
            },
            ExprNode::Var { var_name, var_name_index} => {
                println!("Var(var_name: {}, var_name_index: {})", var_name, var_name_index);
            }
        }
    }
}

impl AstDebugPrinter for AssignmentOperatorType {
    fn debug_visit(&self) {
        match self {
            AssignmentOperatorType::AssignmentDefault => println!("AssignmentDefault"),
            AssignmentOperatorType::AssignmentAdd => println!("AssignmentAdd"),
            AssignmentOperatorType::AssignmentSub => println!("AssignmentSub"),
            AssignmentOperatorType::AssignmentMultiply => println!("AssignmentMultiply"),
            AssignmentOperatorType::AssignmentDivide => println!("AssignmentDivide"),
            AssignmentOperatorType::AssignmentReminder => println!("AssignmentReminder"),
            AssignmentOperatorType::AssignmentBitwiseOr => println!("AssignmentBitwiseOr"),
            AssignmentOperatorType::AssignmentBitwiseAnd => println!("AssignmentBitwiseAnd"),
            AssignmentOperatorType::AssignmentBitwiseXor => println!("AssignmentBitwiseXor"),
            AssignmentOperatorType::AssignmentBitwiseLeftShift => println!("AssignmentBitwiseLeftShift"),
            AssignmentOperatorType::AssignmentBitwiseRightShift => println!("AssignmentBitwiseRightShift"),
        }
    }
}