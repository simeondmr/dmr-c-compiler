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

use std::cmp::PartialEq;
use std::collections::HashMap;

use crate::errors::errors::CompilerErrors;

#[derive(Debug, Clone)]
pub enum SymbolInfo {
    Int,
    Function {
        params_len: usize,
        defined: bool
    }
}

impl PartialEq<SymbolInfo> for &SymbolInfo {
    fn eq(&self, other: &SymbolInfo) -> bool {
        match (self, other) {
            (SymbolInfo::Int, SymbolInfo::Int) => true,
            (SymbolInfo::Function {params_len: _, defined: _ }, SymbolInfo::Function {params_len: _, defined: _ }) => true,
            (_, _) => false
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Linkage {
    External,
    Internal,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Symbol {
    pub name: String,
    pub linkage: Linkage,
    pub symbol_info: SymbolInfo
}

#[derive(Debug)]
pub struct Scope {
    pub symbols: HashMap<String, Symbol>,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub scopes: Vec<Scope>,            // Stack di scope
    pub globals: HashMap<String, Symbol>, // Global/external linkage
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope { symbols: HashMap::new() }],
            globals: HashMap::new(),
        }
    }

    pub fn push_block(&mut self) {
        self.scopes.push(Scope { symbols: HashMap::new() });
    }

    pub fn pop_block(&mut self) {
        self.scopes.pop();
    }

    pub fn add_func_decl(&mut self, symbol: Symbol, is_inside_function: bool) -> Result<(), CompilerErrors> {
        if is_inside_function {
            if let SymbolInfo::Function { params_len, defined } = symbol.symbol_info {
                if defined {
                    eprintln!("Error: nested function are not supported");
                    return Err(CompilerErrors::SemanticError)
                }
                if let Some(current_scope) = self.scopes.last() {
                    let from_current_scope_option = current_scope.symbols.get(&symbol.name);
                    if let Some(from_current_scope) = from_current_scope_option {
                        if let SymbolInfo::Function { params_len: params_len_from_scope, defined: _ } = from_current_scope.symbol_info {
                            if params_len != params_len_from_scope {
                                eprintln!("Error: the declaration of function {} with {} params differs from the previous declaration with {} params in current scope ", symbol.name, params_len, params_len_from_scope);
                                return Err(CompilerErrors::SemanticError)
                            }
                        } else {
                            eprintln!("Error: redeclaration of symbol {} in current scope, with different type", from_current_scope.name);
                            return Err(CompilerErrors::SemanticError)
                        }
                    }
                }
                //Note: add function in current scope and in the global table
                self.scopes.last_mut().map(|scope|scope.symbols.insert(symbol.name.to_string(), symbol.clone()));
                self.check_and_add_func_declaration_global_table(symbol)?;
            }
        } else {
            self.check_and_add_func_declaration_global_table(symbol)?;
        }
        Ok(())
    }

    pub fn add_local_var(&mut self, symbol: Symbol) -> Result<(), CompilerErrors> {
        if let SymbolInfo::Int = symbol.symbol_info {
            let mut current_scope = self.scopes.last_mut();
            if let Some(scope) = &mut current_scope {
                let found_symbol_option = scope.symbols.get(&symbol.name);
                if let Some(found_symbol) = found_symbol_option {
                    eprintln!("Error: redeclared symbol {} in scope", found_symbol.name);
                    return Err(CompilerErrors::SemanticError)
                } else {
                    scope.symbols.insert(symbol.name.to_string(), symbol);
                }
            }
        }
        Ok(())
    }

    fn check_and_add_func_declaration_global_table(&mut self, symbol: Symbol) -> Result<(), CompilerErrors> {
        let mut has_body = false;
        let declaration_from_global_table_option = self.globals.get(&symbol.name);
        if let Some(declaration_from_global_table) = declaration_from_global_table_option {
            if let SymbolInfo::Function { params_len: param_len_from_table, defined: defined_from_table} = declaration_from_global_table.symbol_info {
                has_body = defined_from_table;
                if let SymbolInfo::Function { params_len, defined} = symbol.symbol_info {
                    if defined_from_table && defined {
                        eprintln!(" Error function {} with body redeclared again", symbol.name);
                        return Err(CompilerErrors::SemanticError)
                    }
                    if param_len_from_table != params_len {
                        eprintln!("Error: the declaration of function {} with {} params differs from the previous declaration with {} params", symbol.name, params_len, param_len_from_table);
                        return Err(CompilerErrors::SemanticError)
                    }
                }
            } else {
                eprintln!(" Error redeclaration of symbol {} with different type", symbol.name);
                return Err(CompilerErrors::SemanticError)
            }
        }
        let mut cloned_symbol = symbol.clone();
        if let SymbolInfo::Function { params_len: _, defined} = &mut cloned_symbol.symbol_info {
            *defined |= has_body;
        }
        self.globals.insert(symbol.name.clone(), cloned_symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for scope in self.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
        }
        self.globals.get(name)
    }

    pub fn is_var(&self, name: &str) -> Result<(), CompilerErrors> {
        let symbol = self.lookup(name);
        if let Some(symbol_info) = symbol {
            if let SymbolInfo::Int = symbol_info.symbol_info {
                return Ok(())
            }
        }
        eprintln!("Error: {} is not a variable", name);
        Err(CompilerErrors::SemanticError)
    }

    pub fn is_func(&self, name: &str, args: usize) -> Result<(), CompilerErrors> {
        let symbol = self.lookup(name);
        if let Some(symbol_info) = symbol {
            if let SymbolInfo::Function { params_len, defined: _ } = symbol_info.symbol_info {
                if params_len != args {
                    eprintln!("Error: function was defined with {} params, but the function call is with {} params", params_len, args);
                    return Err(CompilerErrors::SemanticError)
                }
            } else {
                eprintln!("Error: {} is not a function", name);
                return Err(CompilerErrors::SemanticError)
            }
        }
        Ok(())
    }
}