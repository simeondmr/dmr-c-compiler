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

use crate::ast::lang_ast::expr_node::{ExprNode, PrePostOperatorType};
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::expr_parse::ExprParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};
use crate::parser::unop_parse::UnopParse;

pub struct FactorParse {
    unop_parse: UnopParse,
}

impl FactorParse {
    pub fn new() -> FactorParse {
        FactorParse {
            unop_parse: UnopParse::new(),
        }
    }
}

impl GrammarProductionParsing<ExprNode> for FactorParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<ExprNode, CompilerErrors> {
        let current_token = lexer.current_token();
        let factor_node = match current_token {
            Token::NumberU32(value) => {
                lexer.next_token()?;
                Ok(ExprNode::Constant(value as i32))
            },
            Token::BitwiseComplement | Token::Negation | Token::Not => {
                Ok(ExprNode::Unary { unary_operator: self.unop_parse.parse(lexer)?, expr: Box::new(self.parse(lexer)?) })
            },
            Token::RoundBracketOpen => {
                lexer.next_token()?;
                let expr = ExprParse::new();
                let expr_node = expr.parse(lexer, 0, false);
                Self::match_token(&Token::RoundBracketClose, lexer)?;
                expr_node
            },
            Token::Increment | Token::Decrement => {
                lexer.next_token()?;
                let pre_post_operator_type = if let Token::Increment = current_token {
                    PrePostOperatorType::PreIncrement
                } else {
                    PrePostOperatorType::PreDecrement
                };
                if let Token::Literal(var_name) = lexer.current_token() {
                    lexer.next_token()?;
                    Ok(ExprNode::PrePostOperator { pre_post_operator_type, identifier: Box::new(ExprNode::Var { var_name, var_name_index: 0 }) })
                } else {
                    eprintln!("Syntax error at line {:?}: expected literal but found {:?} token", lexer.current_line(), lexer.current_token());
                    return Err(CompilerErrors::SyntaxError)
                }
            },          
            Token::Literal(ref identifier) => {
                lexer.next_token()?;
                let mut args = Vec::new();
                //@Note: function call case
                if let Token::RoundBracketOpen = lexer.current_token() {
                    lexer.next_token()?;
                    if lexer.current_token() != Token::RoundBracketClose {
                        args.push(ExprParse::new().parse(lexer, 0, true)?);
                        while Token::RoundBracketClose != lexer.current_token() {
                            Self::match_token(&Token::Comma, lexer)?;
                            args.push(ExprParse::new().parse(lexer, 0, true)?);
                        }
                    }
                    lexer.next_token()?;
                    Ok(ExprNode::FunctionCall { name: identifier.to_string(), args, has_body: false })
                } else {
                    // Note: During the parsing stage put 0 as identifier_index for every variable. During variable resolution pass the field identifier_index will be fixed with the correct value
                    Ok(ExprNode::Var { var_name: identifier.to_string(), var_name_index: 0 })
                }
            },
            _ => {
                eprintln!("Syntax error at line {:?}: unexpected {:?} token", lexer.current_line(), current_token);
                Err(CompilerErrors::SyntaxError)
            }
        };
        //Note: now we have to check if there is a post inc/dec operator:
        if let Token::Literal(_) = current_token {
            let pre_post_operator = lexer.current_token();
            if FactorParse::is_pre_post_operator(&pre_post_operator) {
                let pre_post_operator_type = if let Token::Increment = pre_post_operator {
                    PrePostOperatorType::PostIncrement
                } else {
                    PrePostOperatorType::PostDecrement
                };
                lexer.next_token()?;
                return Ok(ExprNode::PrePostOperator { pre_post_operator_type, identifier: Box::new(factor_node?) })
            } 
        }
        factor_node
    }
}