use crate::ast::lang_ast::block_node::BlockNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::block_item_parse::BlockItemParse;
use crate::parser::program_parse::GrammarProductionParsing;

pub struct BlockParse {
    block_item: BlockItemParse
}

impl BlockParse {
    pub fn new() -> BlockParse {
        BlockParse {
            block_item: BlockItemParse::new()
        }
    }
}

impl GrammarProductionParsing<BlockNode> for BlockParse {
    fn parse(&self) -> Result<BlockNode, CompilerErrors> {
        let mut lexer =  Self::lexer_lock();
        Self::match_token(&Token::CurlyBracketOpen, &mut lexer)?;
        let mut list_block_item = Vec::new();
        while lexer.current_token() != Token::CurlyBracketClose {
            drop(lexer);
            let current_block_item = self.block_item.parse()?;
            list_block_item.push(current_block_item);
            lexer = Self::lexer_lock();
        }
        lexer.next_token()?;
        Ok(BlockNode::Item(list_block_item))
    }
}