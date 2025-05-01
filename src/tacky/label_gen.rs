use std::sync::{Mutex, OnceLock};

pub static LABEL_GEN_SINGLETON: OnceLock<Mutex<LabelGen>> = OnceLock::new();

pub struct LabelGen {
    label_counter: u32
}

impl LabelGen {
    pub fn new() -> LabelGen {
        LabelGen {
            label_counter: 0
        }
    }
    
    pub fn gen(&mut self) -> u32 {
        let label_gen = self.label_counter;
        self.label_counter += 1;
        label_gen
    }
}