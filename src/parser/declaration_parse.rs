use crate::ast::lang_ast::declaration_node::DeclarationNode;
use crate::ast::lang_ast::expr_node::ExprNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::expr_parse::ExprParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct DeclarationParse;

impl DeclarationParse {
    pub fn new() -> DeclarationParse {
        DeclarationParse {
            
        }
    }
}

impl GrammarProductionParsing<DeclarationNode> for DeclarationParse {
    fn parse(&self) -> Result<DeclarationNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        Self::match_token(&Token::Int, &mut lexer)?;
        let var_name = lexer.current_token();
        Self::match_token(&Token::Literal("".to_string()), &mut lexer)?;
        let assignment_token = lexer.current_token();
        let mut init: Option<ExprNode> = None;
        if let Token::Assignment = assignment_token {
            Self::match_token(&Token::Assignment, &mut lexer)?;
            drop(lexer);
            init = Some(ExprParse::new().parse(0)?);
            lexer = Self::lexer_lock();
        }
        Self::match_token(&Token::Semicolon, &mut lexer)?;
        // Note: During the parsing stage put 0 as var_name_index for every variable. During variable resolution pass the field identifier_index will be fixed with the correct value
        Ok(DeclarationNode::VariableDeclaration { var_name: var_name.extract_literal_val().unwrap(), var_name_index: 0, init })
    }
}