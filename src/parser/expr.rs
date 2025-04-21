use crate::ast::lang_ast::expr_node::ExprNode;
use crate::ast::lang_ast::expr_node::ExprNode::Binary;
use crate::errors::errors::CompilerErrors;
use crate::parser::binop::Binop;
use crate::parser::factor::Factor;
use crate::parser::program::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct Expr {
    factor: Factor,
    binop: Binop
}

impl Expr {
    pub fn new() -> Expr {
        Expr {
            factor: Factor::new(),
            binop: Binop::new()
        }
    }
}

impl PrecedenceClimbingParsing<ExprNode> for Expr {
    fn parse(&self, min_prec: u8) -> Result<ExprNode, CompilerErrors> {
        let mut left_expr = self.factor.parse();
        let mut current_token = Expr::lexer_lock().current_token().clone();
        while Expr::is_operator(&current_token) && Expr::operator_precedence(&current_token)? >= min_prec {
            let operator_precedence = Expr::operator_precedence(&current_token)?;
            let binary_operator = self.binop.parse();
            let right_expr = self.parse(operator_precedence + 1);
            left_expr = Ok(Binary {
                binary_operator: binary_operator?,
                left_expr: Box::new(left_expr?),
                right_expr: Box::new(right_expr?)
            });
            current_token = Expr::lexer_lock().current_token().clone();
        }

        left_expr
    }
}