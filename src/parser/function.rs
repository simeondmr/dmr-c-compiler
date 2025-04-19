use crate::ast::lang_ast::function_node::FunctionNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::program::{GrammarProductionParsing};
use crate::parser::statement::Statement;

pub struct Function {
    statement: Statement
}

impl Function {
    pub fn new() -> Function {
        Function {
            statement: Statement::new()
        }
    }
}

impl GrammarProductionParsing<FunctionNode> for Function {
    fn parse(&self) -> Result<FunctionNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::Int, &mut lexer)?;
        let current_token = lexer.current_token();
        Self::match_token(&Token::Literal("".to_string()), &mut lexer)?;
        let function_name = current_token.extract_literal_val().unwrap();
        Self::match_token(&Token::RoundBracketOpen, &mut lexer)?;
        Self::match_token(&Token::Void, &mut lexer)?;
        Self::match_token(&Token::RoundBracketClose, &mut lexer)?;
        Self::match_token(&Token::CurlyBracketOpen, &mut lexer)?;
        drop(lexer);
        let statement_ast = self.statement.parse()?;
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::CurlyBracketClose, &mut lexer)?;
        Ok(FunctionNode::FunctionDef(function_name, statement_ast))
    }
}