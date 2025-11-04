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

use crate::ast::lang_ast::function_declaration_node::FunctionDeclarationNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::block_parse::BlockParse;
use crate::parser::program_parse::GrammarProductionParsing;

pub struct FunctionDeclarationParse;

impl FunctionDeclarationParse {
    fn parse_param_declaration(args: &mut Vec<String>, lexer: &mut Lexer) -> Result<(), CompilerErrors> {
        Self::match_token(&Token::Int, lexer)?;
        if let Token::Literal(arg) = lexer.current_token() {
            args.push(arg);
            lexer.next_token()?;
            Ok(())
        } else {
            eprintln!("Error: expected literal, found {:?}", lexer.current_token());
            Err(CompilerErrors::SyntaxError)
        }
    }

    fn parse_function_params(lexer: &mut Lexer) -> Result<Vec<String>, CompilerErrors> {
        let mut args = Vec::new();
        if let Token::Void = lexer.current_token() {
            lexer.next_token()?;
        } else {
            Self::parse_param_declaration(&mut args, lexer)?;
            while lexer.current_token() != Token::RoundBracketClose {
                Self::match_token(&Token::Comma, lexer)?;
                Self::parse_param_declaration(&mut args, lexer)?;
            }
        }
        Ok(args)
    }
}

impl GrammarProductionParsing<FunctionDeclarationNode> for FunctionDeclarationParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<FunctionDeclarationNode, CompilerErrors> {
        Self::match_token(&Token::Int, lexer)?;
        let current_token = lexer.current_token();
        Self::match_token(&Token::Literal("".to_string()), lexer)?;
        let func_name = current_token.extract_literal_val().unwrap();
        Self::match_token(&Token::RoundBracketOpen, lexer)?;
        let params = Self::parse_function_params(lexer)?;
        Self::match_token(&Token::RoundBracketClose, lexer)?;
        let block_option  = if let Token::Semicolon = lexer.current_token() {
            lexer.next_token()?;
            None
        } else {
            Some(BlockParse::new().parse(lexer)?)
        };
        Ok(FunctionDeclarationNode::FunctionDef { func_name, params, block_option })
    }
}