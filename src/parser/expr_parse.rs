use crate::ast::lang_ast::expr_node::ExprNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::binop_parse::BinopParse;
use crate::parser::factor_parse::FactorParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct ExprParse {
    factor_parse: FactorParse,
    binop_parse: BinopParse
}

impl ExprParse {
    pub fn new() -> ExprParse {
        ExprParse {
            factor_parse: FactorParse::new(),
            binop_parse: BinopParse::new()
        }
    }
}

impl PrecedenceClimbingParsing<ExprNode> for ExprParse {
    fn parse(&self, min_prec: u8) -> Result<ExprNode, CompilerErrors> {
        let mut left_expr = self.factor_parse.parse();
        let mut current_token = ExprParse::lexer_lock().current_token().clone();
        while ExprParse::is_operator(&current_token) && ExprParse::operator_precedence(&current_token)? >= min_prec {
            if let Some(assignment_operator_type) = ExprParse::is_assignment_operator(&current_token) {
                ExprParse::lexer_lock().next_token()?;
                let operator_precedence = ExprParse::operator_precedence(&current_token)?;
                let right_expr = self.parse(operator_precedence);
                left_expr = Ok(ExprNode::Assignment { assignment_type: assignment_operator_type, dest: Box::new(left_expr?), expr: Box::new(right_expr?) }) 
            } else if let Token::QuestionMark = current_token {
                ExprParse::lexer_lock().next_token()?;
                let true_expr =  Box::new(self.parse(0)?);
                Self::match_token(&Token::Colon, &mut Self::lexer_lock())?;
                let false_expr = Box::new(self.parse(ExprParse::operator_precedence(&current_token)?)?);
                return Ok(ExprNode::Conditional { condition: Box::new(left_expr?), true_expr, false_expr });
            } else {
                let operator_precedence = ExprParse::operator_precedence(&current_token)?;
                let binary_operator = self.binop_parse.parse();
                let right_expr = self.parse(operator_precedence + 1);
                left_expr = Ok(ExprNode::Binary {
                    binary_operator: binary_operator?,
                    left_expr: Box::new(left_expr?),
                    right_expr: Box::new(right_expr?)
                });
            }
            current_token = ExprParse::lexer_lock().current_token().clone();
        }
        left_expr
    }
}