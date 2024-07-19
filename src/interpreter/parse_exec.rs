
use crate::{instructions::instructions_regs::{reg_printall, reg_single_print}, interpreter::decoder::decode, operating_system::memory_manager::get_interval, u4::U4, Machine};

use super::assembler::{parse_line, read_lines_from_file};

pub fn run_programm(virtualm :&mut Machine,filename: &str)-> Result<(), & 'static str> {
   let c = get_interval(filename);
   let mut start = c.0;
   let end = c.1;
   println!("Start  {start} , End {end}");
    let mut l = virtualm.memory.read2(start as u16).unwrap();
    while start < end {
        
     

         l = virtualm.memory.read2( start).unwrap();
         decode(l , virtualm,c.0);
            println!("current : {start}");

         println!("{l}");
        
         reg_printall( virtualm);
        start += 2;
    
    }
        reg_single_print(virtualm, U4::new(15));
    
    Ok(())



}


pub fn parse_programm(virtualm :&mut Machine,filename: &str)-> Result<(), & 'static str> {
let lines =read_lines_from_file(&filename).unwrap();

let c = get_interval(filename);
let mut start = c.0;
let end = c.1;
for line in lines{
    if start < end{
    /*parse_line(&line, &mut virtualm);
    reg_printall(&mut virtualm);    
    virtualm.step()?;*/

    let val = parse_line(&line,   virtualm);

    virtualm.memory.write2(start,val );

         start += 2;
    }
}   
Ok(())

}