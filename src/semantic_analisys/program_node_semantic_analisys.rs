use std::collections::HashMap;
use crate::ast::lang_ast::program_node::ProgramNode;
use crate::errors::errors::CompilerErrors;
use crate::semantic_analisys::resolve_var_expr_trait::ResolveVarExpr;

impl ResolveVarExpr for ProgramNode {
    fn resolve(&mut self, var_map: &mut HashMap<String, u32>) -> Result<(), CompilerErrors> {
        let ProgramNode::ProgramDef(function_def) = self;
        function_def.resolve(var_map)
    }
}