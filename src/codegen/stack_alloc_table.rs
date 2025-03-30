use std::collections::HashMap;

pub struct StackAllocTable {
    current_stack_alloc_value: i32,
    pseudoregisters_address: HashMap<u32, i32>
}

impl StackAllocTable {
    pub fn new() -> StackAllocTable {
        StackAllocTable {
            current_stack_alloc_value: 0,
            pseudoregisters_address: HashMap::new()
        }
    }

    pub fn get_or_insert_pseudoregister(&mut self, pseudoregister_value: u32) -> i32 {
        let pseudoregister_info = self.pseudoregisters_address.get(&pseudoregister_value);
        if let Some(address) = pseudoregister_info {
            return *address;
        }

        self.pseudoregisters_address.insert(pseudoregister_value, self.current_stack_alloc_value);
        let pseudoregister_value = self.current_stack_alloc_value;
        self.current_stack_alloc_value -= 4;
        pseudoregister_value
    }
}