use crate::ast::lang_ast::expr_node::ExprNode;
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
        match current_token {
            Token::NumberU32(value) => {
                lexer.next_token()?;
                Ok(ExprNode::Constant(value as i32))
            },
            Token::BitwiseComplement | Token::Negation | Token::Not => {
                drop(lexer);
                let expr = ExprParse::new();
                Ok(ExprNode::Unary { unary_operator: self.unop_parse.parse()?, expr: Box::new(expr.parse(0)?) })
            },
            Token::RoundBracketOpen => {
                lexer.next_token()?;
                drop(lexer);
                let expr = ExprParse::new();
                let expr_node = expr.parse(0);
                Self::match_token(&Token::RoundBracketClose, &mut Self::lexer_lock())?;
                expr_node
            },
            Token::Literal(var_name) => {
                lexer.next_token()?;
                // Note: During the parsing stage put 0 as identifier_index for every variable. During variable resolution pass the field identifier_index will be fixed with the correct value
                Ok(ExprNode::Var { var_name, var_name_index: 0})
            },
            _ => {
                eprintln!("Syntax error at line {:?}: unexpected {:?} token", lexer.current_line(), current_token);
                Err(CompilerErrors::SyntaxError)
            }
        }
    }
}