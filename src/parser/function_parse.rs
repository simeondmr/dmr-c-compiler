use crate::ast::lang_ast::function_node::FunctionNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::block_item_parse::BlockItemParse;
use crate::parser::program_parse::{GrammarProductionParsing};

pub struct FunctionParse {
    block_item: BlockItemParse
}

impl FunctionParse {
    pub fn new() -> FunctionParse {
        FunctionParse {
            block_item: BlockItemParse::new()
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
        Self::match_token(&Token::CurlyBracketOpen, &mut lexer)?;        
        let mut list_block_item = Vec::new();
        while lexer.current_token() != Token::CurlyBracketClose {
            drop(lexer);
            let current_block_item = self.block_item.parse()?;
            list_block_item.push(current_block_item);
            lexer = Self::lexer_lock();
        }
        lexer.next_token()?;
        Ok(FunctionNode::FunctionDef { func_name, block_item: list_block_item })
    }
}