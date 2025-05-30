use std::collections::HashMap;
use crate::errors::errors::CompilerErrors;

/// This trait need to perform an AST pass in order to check if all goto labels are declared
pub trait CheckGotoLabel {
    fn check_goto_label(&mut self,label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors>;
}