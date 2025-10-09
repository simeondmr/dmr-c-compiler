use crate::ast::lang_ast::statement_node::ForInit;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::declaration_parse::DeclarationParse;
use crate::parser::expr_parse::ExprParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct ForInitParse;

impl ForInitParse {
    pub fn new() -> ForInitParse {
        ForInitParse {

        }
    }
}

impl GrammarProductionParsing<ForInit> for ForInitParse {
    fn parse(&self) -> Result<ForInit, CompilerErrors> {
        let current_token = Self::lexer().lock().unwrap().current_token();
        let result = Ok(if let Token::Int = current_token {
            ForInit::DeclarationInit(DeclarationParse::new().parse()?)
        } else if let Token::Semicolon = current_token {
            Self::lexer().lock().unwrap().next_token()?;
            ForInit::ExpressionInit(None)
        } else {
            let expr_node = ForInit::ExpressionInit(Some(ExprParse::new().parse(0)?));
            Self::match_token(&Token::Semicolon, &mut Self::lexer().lock().unwrap())?;
            expr_node
        });
        result
    }
}
