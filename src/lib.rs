pub mod vm;
pub mod memory;
/* 
pub mod instructions_regs;
pub mod instructions_lin_mem;
pub mod instruction_set;
pub mod interpreter;
pub mod decoder;

pub mod assembler;
*/
pub mod u4;



pub mod instructions {
    pub mod instruction_set;
    pub mod instructions_lin_mem;
    pub mod  instructions_regs;
    
}
pub mod interpreter{
    pub mod assembler;
    pub mod decoder;
}

pub use crate::vm::*;
