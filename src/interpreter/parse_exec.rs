
use crate::{instructions::instructions_regs::{reg_printall, reg_single_print}, interpreter::decoder::decode, u4::U4, Machine};

use super::assembler::{parse_line, read_lines_from_file};

pub fn run_programm(virtualm :&mut Machine,filename: &str)-> Result<(), & 'static str> {
  let _ =  parse_programm(virtualm, filename);
    
    let mut l = virtualm.memory.read2(0 as u16).unwrap();
    while l != 0 {
        decode(l as u16, virtualm);
       
        virtualm.registers[13] += 2; 
         
         l = virtualm.memory.read2(  virtualm.registers[13]as u16).unwrap();
         println!("{l}");
         let _ =  virtualm.step();
         reg_printall( virtualm);

    }

        reg_single_print(virtualm, U4::new(15));
    
    Ok(())



}


pub fn parse_programm(virtualm :&mut Machine,filename: &str)-> Result<(), & 'static str> {
let lines =read_lines_from_file(&filename).unwrap();
let mut index= 0;
for line in lines{
    /*parse_line(&line, &mut virtualm);
    reg_printall(&mut virtualm);    
    virtualm.step()?;*/

    let val = parse_line(&line,   virtualm);

    virtualm.memory.write2(index,val );

        index += 2;

}   
Ok(())

}