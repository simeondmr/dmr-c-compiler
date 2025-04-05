use std::io::Error;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CompilerErrors {
    LexicalError,
    SyntaxError,
    WrongParams,
    IO(Error)
}