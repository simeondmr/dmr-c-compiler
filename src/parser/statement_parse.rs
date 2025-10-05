use std::collections::HashMap;
use crate::ast::lang_ast::statement_node::{LoopLabels, StatementNode};
use crate::errors::errors::CompilerErrors;
use crate::lexer::lexer::Token;
use crate::parser::block_parse::BlockParse;
use crate::parser::expr_parse::ExprParse;
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
    fn parse(&self) -> Result<StatementNode, CompilerErrors> {
        let mut lexer = Self::lexer_lock();
        let current_token = lexer.current_token();
        match current_token {
            Token::If => {
               lexer.next_token()?;
               Self::match_token(&Token::RoundBracketOpen, &mut lexer)?;
               drop(lexer);
               let expr_node  = self.expr_parse.parse(0)?;
               Self::match_token(&Token::RoundBracketClose, &mut Self::lexer_lock())?;
               let stmt_node = self.parse()?;
               let mut lexer = Self::lexer_lock();
               let mut else_node = None;
               if let Token::Else = lexer.current_token() {
                   lexer.next_token()?;
                   drop(lexer);
                   else_node = Some(Box::new(self.parse()?));
               }
               Ok(StatementNode::IfStmt { condition: expr_node, stmt: Box::new(stmt_node), else_stmt: else_node })
            },
            Token::Return => {
                lexer.next_token()?;
                drop(lexer);
                let expr_node  = self.expr_parse.parse(0)?;
                let mut lexer = Self::lexer_lock();
                Self::match_token(&Token::Semicolon, &mut lexer)?;
                Ok(StatementNode::ReturnStmt(expr_node))
            },
            Token::While => {
                lexer.next_token()?;
                Self::match_token(&Token::RoundBracketOpen, &mut lexer)?;
                drop(lexer);
                let condition_node = self.expr_parse.parse(0)?;
                Self::match_token(&Token::RoundBracketClose, &mut Self::lexer_lock())?;
                let stmt_node = self.parse()?;
                Ok(StatementNode::WhileStmt { condition: condition_node, stmt: Box::new(stmt_node), loop_labels: LoopLabels::new() })
            },
            Token::Do => {
                lexer.next_token()?;
                drop(lexer);
                let stmt_node = self.parse()?;
                Self::match_token(&Token::While, &mut Self::lexer_lock())?;
                Self::match_token(&Token::RoundBracketOpen, &mut Self::lexer_lock())?;
                let condition_node = self.expr_parse.parse(0)?;
                Self::match_token(&Token::RoundBracketClose, &mut Self::lexer_lock())?;
                Self::match_token(&Token::Semicolon, &mut Self::lexer_lock())?;
                Ok(StatementNode::DoWhileStmt { condition: condition_node, stmt: Box::new(stmt_node), loop_labels: LoopLabels::new() })
            },
            Token::Goto => {
                lexer.next_token()?;
                if let Token::Literal(label_name) = lexer.current_token() {
                    lexer.next_token()?;
                    Self::match_token(&Token::Semicolon, &mut lexer)?;
                    Ok(StatementNode::Goto { label_name, label_name_index: 0})
                } else {
                    eprintln!("Syntax error: expected literal in goto statement");
                    Err(CompilerErrors::SyntaxError)
                }
            },
            Token::Break => {
                lexer.next_token()?;
                Self::match_token(&Token::Semicolon, &mut lexer)?;
                Ok(StatementNode::BreakStmt(None))
            },
            Token::Continue => {
                lexer.next_token()?;
                Self::match_token(&Token::Semicolon, &mut lexer)?;
                Ok(StatementNode::ContinueStmt(None))
            },
            Token::Switch => {
                lexer.next_token()?;
                Self::match_token(&Token::RoundBracketOpen, &mut lexer)?;
                drop(lexer);
                let switch_value_node = self.expr_parse.parse(0)?;
                Self::match_token(&Token::RoundBracketClose, &mut Self::lexer_lock())?;
                Ok(StatementNode::SwitchStmt { condition: switch_value_node, stmt: Box::new(self.parse()?), loop_labels: LoopLabels::new(), cases_map: HashMap::new(), default_stmt_label: None })
            },
            Token::Case => {
                lexer.next_token()?;
                drop(lexer);
                let case_label = self.expr_parse.parse(0)?;
                Self::match_token(&Token::Colon, &mut Self::lexer_lock())?;
                Ok(StatementNode::CaseStmt { label: 0, value: case_label, stmt: Box::new(self.parse()?)})
            },
            Token::Default => {
                lexer.next_token()?;
                drop(lexer);
                Self::match_token(&Token::Colon, &mut Self::lexer_lock())?;
                Ok(StatementNode::DefaultStmt { label: 0, stmt: Box::new(self.parse()?) })
            },
            Token::CurlyBracketOpen => {
                drop(lexer);
                Ok(StatementNode::Compound(BlockParse::new().parse()?))
            },
            Token::Semicolon => {
                lexer.next_token()?;
                Ok(StatementNode::EmptyStmt)
            },
            _ => {
                if let Token::Literal(name) = current_token {
                    lexer.feed_lookeheads_tokens(1)?;
                    let peek = lexer.peek_lookaheader_token(0);
                    if let Some(Token::Colon) = peek {
                        lexer.remove_lookahead()?;
                        drop(lexer);
                        let stmt_node = self.parse()?;
                        return Ok(StatementNode::LabelStmt { label_name: name, label_name_index: 0, stmt: Box::new(stmt_node) })
                   }
                }
                // Note: if is not a return stmt and neither an Empty stmt so must be an expression
                drop(lexer);
                let expr_ast = self.expr_parse.parse(0)?;
                let mut lexer = Self::lexer_lock();
                Self::match_token(&Token::Semicolon, &mut lexer)?;
                Ok(StatementNode::Expr(expr_ast))
            }
        }
    }
}