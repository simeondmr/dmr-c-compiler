use std::fs::File;
use std::io::Error;

pub trait Codegen {
    fn codegen(&self, output_file: &mut File) -> Result<(), Error>;
}