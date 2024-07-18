

use decoder::decode;
use assembler::{parse_line, read_lines_from_file};
use instructions_regs::{reg_add, reg_immediate_add, reg_jump, reg_load_word, reg_printall, reg_single_print, reg_store_word};
// Virtual Machine Binary 
use rust_projects::*;
use rust_projects::instructions_lin_mem::*;
use rust_projects::u4::U4;

pub fn main()-> Result<(), & 'static str> {
    let mut virtualm = Machine::new();
    reg_immediate_add(&mut virtualm, U4::new(1), U4::new(1), U4::new(5));
    virtualm.step()?;
    reg_immediate_add(&mut virtualm, U4::new(2), U4::new(2), U4::new(7));
    virtualm.step()?;
    println!("--------------------------------------");

    let lines =read_lines_from_file("Assembly.txt").unwrap();
    for line in lines{
        parse_line(&line, &mut virtualm);
        reg_printall(&mut virtualm);

    }


    virtualm.step()

}
