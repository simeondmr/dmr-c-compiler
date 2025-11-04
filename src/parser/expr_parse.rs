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
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
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
    fn parse(&self, lexer: &mut Lexer, min_prec: u8, comma_stop: bool) -> Result<ExprNode, CompilerErrors> {
        let mut left_expr = self.factor_parse.parse(lexer);
        let mut current_token = lexer.current_token().clone();
        while ExprParse::is_operator(&current_token) && ExprParse::operator_precedence(&current_token)? >= min_prec {
            //Note: in order to parse correctly functions params and comma operator
            if comma_stop && current_token == Token::Comma {
                break;
            }
            if let Some(assignment_operator_type) = ExprParse::is_assignment_operator(&current_token) {
                lexer.next_token()?;
                let operator_precedence = ExprParse::operator_precedence(&current_token)?;
                let right_expr = self.parse(lexer, operator_precedence, comma_stop);
                left_expr = Ok(ExprNode::Assignment { assignment_type: assignment_operator_type, dest: Box::new(left_expr?), expr: Box::new(right_expr?) }) 
            } else if let Token::QuestionMark = current_token {
                lexer.next_token()?;
                let true_expr =  Box::new(self.parse(lexer, 0, comma_stop)?);
                Self::match_token(&Token::Colon, lexer)?;
                let false_expr = Box::new(self.parse(lexer, ExprParse::operator_precedence(&current_token)?, comma_stop)?);
                return Ok(ExprNode::Conditional { condition: Box::new(left_expr?), true_expr, false_expr });
            } else {
                let operator_precedence = ExprParse::operator_precedence(&current_token)?;
                let binary_operator = self.binop_parse.parse(lexer);
                let right_expr = self.parse(lexer, operator_precedence + 1, comma_stop);
                left_expr = Ok(ExprNode::Binary {
                    binary_operator: binary_operator?,
                    left_expr: Box::new(left_expr?),
                    right_expr: Box::new(right_expr?)
                });
            }
            current_token = lexer.current_token().clone();
        }
        left_expr
    }
}