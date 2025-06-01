use crate::ast::lang_ast::function_node::FunctionNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::block_parse::BlockParse;
use crate::parser::program_parse::{GrammarProductionParsing};

pub struct FunctionParse {
    block: BlockParse
}

impl FunctionParse {
    pub fn new() -> FunctionParse {
        FunctionParse {
            block: BlockParse::new()
        }
    }
}

impl GrammarProductionParsing<FunctionNode> for FunctionParse {
    fn parse(&self) -> Result<FunctionNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::Int, &mut lexer)?;
        let current_token = lexer.current_token();
        Self::match_token(&Token::Literal("".to_string()), &mut lexer)?;
        let func_name = current_token.extract_literal_val().unwrap();
        Self::match_token(&Token::RoundBracketOpen, &mut lexer)?;
        Self::match_token(&Token::Void, &mut lexer)?;
        Self::match_token(&Token::RoundBracketClose, &mut lexer)?;
        drop(lexer);
        Ok(FunctionNode::FunctionDef { func_name, block: self.block.parse()? })
    }
}