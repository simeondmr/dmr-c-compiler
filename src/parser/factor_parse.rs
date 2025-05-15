use crate::ast::lang_ast::expr_node::{ExprNode, PrePostOperatorType};
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
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
    fn parse(&self) -> Result<ExprNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();
        let factor_node = match current_token {
            Token::NumberU32(value) => {
                lexer.next_token()?;
                drop(lexer);
                Ok(ExprNode::Constant(value as i32))
            },
            Token::BitwiseComplement | Token::Negation | Token::Not => {
                drop(lexer);
                Ok(ExprNode::Unary { unary_operator: self.unop_parse.parse()?, expr: Box::new(self.parse()?) })
            },
            Token::RoundBracketOpen => {
                lexer.next_token()?;
                drop(lexer);
                let expr = ExprParse::new();
                let expr_node = expr.parse(0);
                Self::match_token(&Token::RoundBracketClose, &mut Self::lexer_lock())?;
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
                    drop(lexer);
                    Ok(ExprNode::PrePostOperator { pre_post_operator_type, identifier: Box::new(ExprNode::Var { var_name, var_name_index: 0 }) })
                } else {
                    eprintln!("Syntax error at line {:?}: expected literal but found {:?} token", lexer.current_line(), lexer.current_token());
                    return Err(CompilerErrors::SyntaxError)
                }
            },          
            Token::Literal(ref var_name) => {
                lexer.next_token()?;
                drop(lexer);
                // Note: During the parsing stage put 0 as identifier_index for every variable. During variable resolution pass the field identifier_index will be fixed with the correct value
                Ok(ExprNode::Var {var_name: var_name.to_string(), var_name_index: 0})
            },
            _ => {
                eprintln!("Syntax error at line {:?}: unexpected {:?} token", lexer.current_line(), current_token);
                Err(CompilerErrors::SyntaxError)
            }
        };
        //Note: now we have to check if there is a post inc/dec operator:
        if let Token::Literal(_) = current_token {
            let mut lexer = Self::lexer_lock();
            let pre_post_operator = lexer.current_token();
            if FactorParse::is_pre_post_operator(&pre_post_operator) {
                let pre_post_operator_type = if let Token::Increment = pre_post_operator {
                    PrePostOperatorType::PostIncrement
                } else {
                    PrePostOperatorType::PostDecrement
                };
                lexer.next_token()?;
                drop(lexer);
                return Ok(ExprNode::PrePostOperator { pre_post_operator_type, identifier: Box::new(factor_node?) })
            } 
        }
        factor_node
    }
}