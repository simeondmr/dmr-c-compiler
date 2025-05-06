use std::collections::HashMap;
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExpr;

pub fn semantic_analisys(program_node: &mut ProgramNode) -> Result<(), CompilerErrors> {
    let mut var_map: HashMap<String, u32> = HashMap::new();
    program_node.resolve(&mut var_map)
}