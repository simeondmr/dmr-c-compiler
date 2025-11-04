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