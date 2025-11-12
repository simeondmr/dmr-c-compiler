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

use crate::ast::lang_ast::expr_node::AssignmentOperatorType;
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::function_declaration_parse::FunctionDeclarationParse;

pub trait GrammarProductionParsing<T> {
    fn parse(&self, lexer: &mut Lexer) -> Result<T, CompilerErrors>;

    fn match_token(expected_token: &Token, lexer: &mut Lexer) -> Result<(), CompilerErrors> {
        if *expected_token == lexer.current_token() {
            lexer.next_token()?;
            return Ok(())
        }

        eprintln!("Error at line {}: expected {:?}, but found {:?}", lexer.current_line(), expected_token, lexer.current_token());
        Err(CompilerErrors::SyntaxError)
    }

    fn is_pre_post_operator(operator: &Token) -> bool {
        match operator {
            Token::Increment => true,
            Token::Decrement => true,
            _ => false
        }
    }
}

#[allow(dead_code)]
pub trait PrecedenceClimbingParsing<T> {
    fn parse(&self, lexer: &mut Lexer, min_prec: u8, comma_stop: bool) -> Result<T, CompilerErrors>;

    fn match_token(expected_token: &Token, lexer: &mut Lexer) -> Result<(), CompilerErrors> {
        if *expected_token == lexer.current_token() {
            lexer.next_token()?;
            return Ok(())
        }

        eprintln!("Error at line {}: expected {:?}, but found {:?}", lexer.current_line(), expected_token, lexer.current_token());
        Err(CompilerErrors::SyntaxError)
    }

    fn is_operator(operator: &Token) -> bool {
        match operator {
            Token::BitwiseComplement => true,
            Token::Negation => true,
            Token::Add => true,
            Token::Multiply => true,
            Token::Divide => true,
            Token::Reminder => true,
            Token::BitwiseAnd => true,
            Token::BitwiseOr => true,
            Token::BitwiseXor => true,
            Token::BitwiseLeftShift => true,
            Token::BitwiseRightShift => true,
            Token::Equal => true,
            Token::NotEqual => true,
            Token::LessThan => true,
            Token::LessThanOrEqual => true,
            Token::GreaterThan => true,
            Token::GreaterThanOrEqual => true,
            Token::And => true,
            Token::Or => true,
            Token::Not => true,
            Token::Assignment => true,
            Token::AssignmentAdd => true,
            Token::AssignmentSub => true,
            Token::AssignmentMultiply => true,
            Token::AssignmentDivide => true,
            Token::AssignmentReminder => true,
            Token::AssignmentBitwiseOr => true,
            Token::AssignmentBitwiseAnd => true,
            Token::AssignmentBitwiseXor => true,
            Token::AssignmentBitwiseLeftShift => true,
            Token::AssignmentBitwiseRightShift => true,
            Token::Comma => true,
            Token::QuestionMark => true,
            _ => false
        }
    }
    
    fn is_assignment_operator(operator: &Token) -> Option<AssignmentOperatorType> {
        match operator {
            Token::Assignment => Some(AssignmentOperatorType::AssignmentDefault),
            Token::AssignmentAdd => Some(AssignmentOperatorType::AssignmentAdd),
            Token::AssignmentSub => Some(AssignmentOperatorType::AssignmentSub),
            Token::AssignmentMultiply => Some(AssignmentOperatorType::AssignmentMultiply),
            Token::AssignmentDivide => Some(AssignmentOperatorType::AssignmentDivide),
            Token::AssignmentReminder => Some(AssignmentOperatorType::AssignmentReminder),
            Token::AssignmentBitwiseOr => Some(AssignmentOperatorType::AssignmentBitwiseOr),
            Token::AssignmentBitwiseAnd => Some(AssignmentOperatorType::AssignmentBitwiseAnd),
            Token::AssignmentBitwiseXor => Some(AssignmentOperatorType::AssignmentBitwiseXor),
            Token::AssignmentBitwiseLeftShift => Some(AssignmentOperatorType::AssignmentBitwiseLeftShift),
            Token::AssignmentBitwiseRightShift => Some(AssignmentOperatorType::AssignmentBitwiseRightShift),
            _ => None
        }
    }

    fn operator_precedence(operator: &Token) -> Result<u8, CompilerErrors> {
        match operator {
            Token::Comma => Ok(1),
            Token::BitwiseComplement => Ok(60),
            Token::Negation => Ok(50),
            Token::Add => Ok(50),
            Token::Multiply => Ok(60),
            Token::Divide => Ok(60),
            Token::Reminder => Ok(60),
            Token::BitwiseAnd => Ok(45),
            Token::BitwiseOr => Ok(45),
            Token::BitwiseXor => Ok(45),
            Token::BitwiseLeftShift => Ok(55),
            Token::BitwiseRightShift => Ok(55),
            Token::Equal => Ok(30),
            Token::NotEqual => Ok(30),
            Token::LessThan => Ok(35),
            Token::LessThanOrEqual => Ok(35),
            Token::GreaterThan => Ok(35),
            Token::GreaterThanOrEqual => Ok(35),
            Token::And => Ok(10),
            Token::Or => Ok(5),
            Token::QuestionMark => Ok(3),
            Token::Assignment => Ok(1),
            Token::AssignmentAdd => Ok(1),
            Token::AssignmentSub => Ok(1),
            Token::AssignmentMultiply => Ok(1),
            Token::AssignmentDivide => Ok(1),
            Token::AssignmentReminder => Ok(1),
            Token::AssignmentBitwiseOr => Ok(1),
            Token::AssignmentBitwiseAnd => Ok(1),
            Token::AssignmentBitwiseXor => Ok(1),
            Token::AssignmentBitwiseLeftShift => Ok(1),
            Token::AssignmentBitwiseRightShift => Ok(1),
            _ => Err(CompilerErrors::OperatorPrecedenceError)
        }
    }
}

pub struct ProgramParse {
    
}

impl ProgramParse {
    pub fn new() -> ProgramParse {
        ProgramParse {

        }
    }
}

impl GrammarProductionParsing<ProgramNode> for ProgramParse {
    fn parse(&self, lexer: &mut Lexer) -> Result<ProgramNode, CompilerErrors> {
        let mut functions = Vec::new();
        lexer.next_token()?;
        while lexer.current_token() != Token::Eof {
            functions.push(FunctionDeclarationParse.parse(lexer)?);
        }
        Self::match_token(&Token::Eof, lexer)?;
        Ok(ProgramNode::ProgramDef(functions))
    }
}