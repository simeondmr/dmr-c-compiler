use std::collections::HashMap;
use crate::errors::errors::CompilerErrors;

pub struct SymbolTable {
    block_stack: Vec<HashMap<String, u32>>
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            block_stack: Vec::new()
        }
    }
    
    pub fn push_block(&mut self) {
        self.block_stack.push(HashMap::new())
    }
    
    pub fn pop_block(&mut self) {
        let _ = self.block_stack.pop();
    }

    pub fn new_variable(&mut self, var_name: String, var_index: u32) -> Result<(), CompilerErrors> {
        if let Some(current_block) = self.block_stack.last_mut() {
            if current_block.contains_key(&var_name) {
                eprintln!("Error: redeclaration of variable {}", var_name);
                return Err(CompilerErrors::SemanticError)
            }
            current_block.insert(var_name.to_string(), var_index);
        } else {
            // Note: this case is impossible
            eprintln!("Error: no new block declared");
            return Err(CompilerErrors::SemanticError)
        }
        Ok(())
    }
    
    pub fn var_index(&self, var_name: &String) -> Option<u32> {
        for block in self.block_stack.iter().rev() {
            if let Some(var_index) = block.get(var_name) {
                return Some(*var_index)
            }
        }
        None
    }
}