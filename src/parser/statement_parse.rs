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

use std::collections::HashMap;
use crate::ast::lang_ast::statement_node::{LoopLabels, StatementNode};
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::{Lexer, Token};
use crate::parser::block_parse::BlockParse;
use crate::parser::expr_parse::ExprParse;
use crate::parser::for_init_parse::ForInitParse;
use crate::parser::program_parse::{GrammarProductionParsing, PrecedenceClimbingParsing};

pub struct Statement {
    expr_parse: ExprParse
}

impl Statement {
    pub fn new() -> Statement {
        Statement {
            expr_parse: ExprParse::new()
        }
    }
}

impl GrammarProductionParsing<StatementNode> for Statement {
    fn parse(&self, lexer: &mut Lexer) -> Result<StatementNode, CompilerErrors> {
        let current_token = lexer.current_token();
        match current_token {
            Token::If => {
               lexer.next_token()?;
               Self::match_token(&Token::RoundBracketOpen, lexer)?;
               let expr_node  = self.expr_parse.parse(lexer, 0, false)?;
               Self::match_token(&Token::RoundBracketClose, lexer)?;
               let stmt_node = self.parse(lexer)?;
               let mut else_node = None;
               if let Token::Else = lexer.current_token() {
                   lexer.next_token()?;
                   else_node = Some(Box::new(self.parse(lexer)?));
               }
               Ok(StatementNode::IfStmt { condition: expr_node, stmt: Box::new(stmt_node), else_stmt: else_node })
            },
            Token::Return => {
                lexer.next_token()?;
                let expr_node  = self.expr_parse.parse(lexer, 0, false)?;
                Self::match_token(&Token::Semicolon, lexer)?;
                Ok(StatementNode::ReturnStmt(expr_node))
            },
            Token::While => {
                lexer.next_token()?;
                Self::match_token(&Token::RoundBracketOpen, lexer)?;
                let condition_node = self.expr_parse.parse(lexer, 0, false)?;
                Self::match_token(&Token::RoundBracketClose, lexer)?;
                let stmt_node = self.parse(lexer)?;
                Ok(StatementNode::WhileStmt { condition: condition_node, stmt: Box::new(stmt_node), loop_labels: LoopLabels::new() })
            },
            Token::For => {
                lexer.next_token()?;
                Self::match_token(&Token::RoundBracketOpen, lexer)?;
                let for_init_node = ForInitParse::new().parse(lexer)?;
                let current_token = lexer.current_token();
                let condition_node =  if let Token::Semicolon = current_token {
                    None
                } else {
                    Some(self.expr_parse.parse(lexer, 0, false)?)
                };
                Self::match_token(&Token::Semicolon, lexer)?;
                let next_token = lexer.current_token();
                let post_expr_node = if let Token::RoundBracketClose = next_token {
                    None
                } else {
                    Some(self.expr_parse.parse(lexer, 0, false)?)
                };
                Self::match_token(&Token::RoundBracketClose, lexer)?;
                let stmt_node = self.parse(lexer)?;
                Ok(StatementNode::ForStmt { init: for_init_node, condition: condition_node, post_expr: post_expr_node, stmt: Box::new(stmt_node), loop_labels: LoopLabels::new() })
            },
            Token::Do => {
                lexer.next_token()?;
                let stmt_node = self.parse(lexer)?;
                Self::match_token(&Token::While, lexer)?;
                Self::match_token(&Token::RoundBracketOpen, lexer)?;
                let condition_node = self.expr_parse.parse(lexer, 0, false)?;
                Self::match_token(&Token::RoundBracketClose, lexer)?;
                Self::match_token(&Token::Semicolon, lexer)?;
                Ok(StatementNode::DoWhileStmt { condition: condition_node, stmt: Box::new(stmt_node), loop_labels: LoopLabels::new() })
            },
            Token::Goto => {
                lexer.next_token()?;
                if let Token::Literal(label_name) = lexer.current_token() {
                    lexer.next_token()?;
                    Self::match_token(&Token::Semicolon, lexer)?;
                    Ok(StatementNode::Goto { label_name, label_name_index: 0})
                } else {
                    eprintln!("Syntax error: expected literal in goto statement");
                    Err(CompilerErrors::SyntaxError)
                }
            },
            Token::Break => {
                lexer.next_token()?;
                Self::match_token(&Token::Semicolon, lexer)?;
                Ok(StatementNode::BreakStmt(None))
            },
            Token::Continue => {
                lexer.next_token()?;
                Self::match_token(&Token::Semicolon, lexer)?;
                Ok(StatementNode::ContinueStmt(None))
            },
            Token::Switch => {
                lexer.next_token()?;
                Self::match_token(&Token::RoundBracketOpen, lexer)?;
                let switch_value_node = self.expr_parse.parse(lexer, 0, false)?;
                Self::match_token(&Token::RoundBracketClose, lexer)?;
                Ok(StatementNode::SwitchStmt { condition: switch_value_node, stmt: Box::new(self.parse(lexer)?), loop_labels: LoopLabels::new(), cases_map: HashMap::new(), default_stmt_label: None })
            },
            Token::Case => {
                lexer.next_token()?;
                let case_label = self.expr_parse.parse(lexer, 0, false)?;
                Self::match_token(&Token::Colon, lexer)?;
                Ok(StatementNode::CaseStmt { label: 0, value: case_label, stmt: Box::new(self.parse(lexer)?)})
            },
            Token::Default => {
                lexer.next_token()?;
                Self::match_token(&Token::Colon, lexer)?;
                Ok(StatementNode::DefaultStmt { label: 0, stmt: Box::new(self.parse(lexer)?) })
            },
            Token::CurlyBracketOpen => {
                Ok(StatementNode::Compound(BlockParse::new().parse(lexer)?))
            },
            Token::Semicolon => {
                lexer.next_token()?;
                Ok(StatementNode::EmptyStmt)
            },
            _ => {
                if let Token::Literal(name) = current_token {
                    lexer.feed_lookaheads_tokens(1)?;
                    let peek = lexer.peek_lookaheader_token(0);
                    if let Some(Token::Colon) = peek {
                        lexer.remove_lookahead()?;
                        let stmt_node = self.parse(lexer)?;
                        return Ok(StatementNode::LabelStmt { label_name: name, label_name_index: 0, stmt: Box::new(stmt_node) })
                   }
                }
                // Note: if is not a return stmt and neither an Empty stmt so must be an expression
                let expr_ast = self.expr_parse.parse(lexer, 0, false)?;
                Self::match_token(&Token::Semicolon, lexer)?;
                Ok(StatementNode::Expr(expr_ast))
            }
        }
    }
}