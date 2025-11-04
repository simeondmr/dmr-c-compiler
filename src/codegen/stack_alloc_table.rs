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

pub struct StackAllocTable {
    stack_offset: i32,
    pseudoregisters_address: HashMap<u32, i32>
}

impl StackAllocTable {
    pub fn new() -> StackAllocTable {
        StackAllocTable {
            stack_offset: 0,
            pseudoregisters_address: HashMap::new()
        }
    }

    pub fn get_or_insert_pseudoregister(&mut self, pseudoregister_value: u32) -> i32 {
        let pseudoregister_info = self.pseudoregisters_address.get(&pseudoregister_value);
        if let Some(address) = pseudoregister_info {
            return *address;
        }
        self.pseudoregisters_address.insert(pseudoregister_value, self.stack_offset);
        let pseudoregister_value = self.stack_offset;
        self.stack_offset -= 4;
        pseudoregister_value
    }
    
    pub fn stack_offset(&self) -> i32 {
        self.stack_offset
    }
}