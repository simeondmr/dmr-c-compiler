use std::collections::HashMap;
use crate::errors::errors::CompilerErrors;

pub trait ResolveVarExprLabel {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>, label_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors>;
}