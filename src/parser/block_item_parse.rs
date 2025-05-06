use crate::ast::lang_ast::block_item_node::BlockItemNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::declaration_parse::DeclarationParse;
use crate::parser::program_parse::GrammarProductionParsing;
use crate::parser::statement_parse::Statement;

pub struct BlockItemParse;

impl BlockItemParse {
    pub fn new() -> BlockItemParse {
        BlockItemParse {
            
        }
    }
}

impl GrammarProductionParsing<BlockItemNode> for BlockItemParse {
    fn parse(&self) -> Result<BlockItemNode, CompilerErrors> {
        let current_token = Self::lexer_lock().current_token();
        if let Token::Int = current_token {
            Ok(BlockItemNode::Declaration(DeclarationParse::new().parse()?))
        } else {
            Ok(BlockItemNode::Statement(Statement::new().parse()?))
        }
    }
}