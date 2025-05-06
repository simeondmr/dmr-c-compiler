use std::io::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CompilerErrors {
    LexicalError,
    SyntaxError,
    SemanticError,
    WrongParams,
    OperatorPrecedenceError,
    IO(Error)
}