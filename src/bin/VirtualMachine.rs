




use operating_system::user_interface::{self, cmd_line_interface};
use instructions::instructions_regs;
use instructions_regs::*;
use interpreter::assembler::*;
use interpreter::decoder::decode;
// Virtual Machine Binary 
use rust_projects::*;


pub fn main()-> Result<(), & 'static str> {
    let mut virtualm = Machine::new();  
    
    cmd_line_interface(&mut virtualm);

    virtualm.step()
}
