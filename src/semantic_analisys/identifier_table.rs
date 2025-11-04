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
use crate::errors::errors::CompilerErrors;

#[derive(Debug)]
pub struct IdentifierInfo {
    address: u32,
    has_external_link: bool
}

impl IdentifierInfo {
    fn new(address: u32, has_external_link: bool) -> IdentifierInfo {
        IdentifierInfo {
            address,
            has_external_link
        }
    }

    fn address(&self) -> u32 {
        self.address
    }

    fn has_external_link(&self) -> bool {
        self.has_external_link
    }

    fn set_address(&mut self, address: u32) {
        self.address = address
    }

    fn set_has_external_link(&mut self, has_external_link: bool) {
        self.has_external_link = has_external_link
    }
}

pub struct IdentifierTable {
    block_stack: Vec<HashMap<String, IdentifierInfo>>
}

impl IdentifierTable {
    pub fn new() -> IdentifierTable {
        let mut identifier_table = IdentifierTable {
            block_stack: Vec::new()
        };
        // Note that this block is needed for global identifiers
        identifier_table.push_block();
        identifier_table
    }
    
    pub fn push_block(&mut self) {
        self.block_stack.push(HashMap::new())
    }
    
    pub fn pop_block(&mut self) {
        let _ = self.block_stack.pop();
    }

    pub fn new_local_variable(&mut self, var_name: String, var_index: u32) -> Result<(), CompilerErrors> {
        if let Some(current_block) = self.block_stack.last_mut() {
            if current_block.contains_key(&var_name) {
                eprintln!("Error: redeclaration of variable {}", var_name);
                return Err(CompilerErrors::SemanticError)
            }
            current_block.insert(var_name.to_string(), IdentifierInfo::new(var_index, false));
        } else {
            // Note: this case is impossible
            eprintln!("Error: no new block declared");
            return Err(CompilerErrors::SemanticError)
        }
        Ok(())
    }

    pub fn insert_ext_link_identifier(&mut self, identifier: String) -> Result<(), CompilerErrors> {
        if let Some(global_block) = self.block_stack.get_mut(0) {
            let identifier_option = global_block.get(&identifier);
            if let Some(identifier_info) = identifier_option {
                if !identifier_info.has_external_link() {
                    eprintln!("Error: redeclaration of external linkage {}", identifier);
                    return Err(CompilerErrors::SemanticError)
                }
            }
            global_block.insert(identifier.to_string(), IdentifierInfo::new(0, true));
        } else {
            // Note: this case is impossible
            eprintln!("Error: no new block declared");
            return Err(CompilerErrors::SemanticError)
        }
        Ok(())
    }

    pub fn ext_identifier(&self, name: String) -> Result<&IdentifierInfo, CompilerErrors> {
        if let Some(global_block) = self.block_stack.get(0) {
            if let Some(identifier) = global_block.get(&name) {
                return Ok(identifier);
            }
        }
        println!("Error: identifier {} not found", name);
        Err(CompilerErrors::SemanticError)
    }

    pub fn var_index(&self, var_name: &String) -> Option<u32> {
        println!("Hashmap for var {:?}", self.block_stack);
        for block in self.block_stack.iter().rev() {
            if let Some(identifier_info) = block.get(var_name) {
                return Some(identifier_info.address())
            }
        }
        None
    }
}