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

use crate::tacky::tacky_instruction_node::InstructionTackyNode;

pub trait AstDebugPrinter {
    fn debug_visit(&self);
}

pub trait GenerateTacky<T> {
    fn to_tacky(&self) -> T;
}

pub trait GenerateTackyInstructions<T> {
    fn to_tacky(&self, tacky_instructions: &mut Vec<InstructionTackyNode>) -> T;
}