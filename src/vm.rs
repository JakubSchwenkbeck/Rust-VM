
/** Building a 16Bit Virtual Machine in Rust, with help of https://www.youtube.com/watch?v=oArXOAhzOdY
 *  Goals: Firstly learn Rust syntax and concepts, then might really do a VM in the end:)
 */

use crate::memory::{LinMem,Addressable};

pub enum Register{
   A,B,C,D, Arg1,Arg2,Val1, Val2,Temp1, Temp2,Temp3,Temp4, SP, PC, BP, FLAGS,  // Registers like Stack pointer, programmcounter etc
} 

pub enum Operations{
    Nop,
}


pub struct Machine{ 
     pub registers: [u16; 16] , // array of our registers
     pub memory : Box<dyn Addressable>,
     // first version of memory Box puts it on the heap instead of stack, might change later
}

impl Machine{ // creates a machine
    pub fn new() -> Self { // 
        Self{
            registers:[0;16],
            memory: Box::new(LinMem::new(8 * 1024)), // init mem with 8kb
        }
    }

    pub fn step(&mut self)-> Result<(),&'static str> { // Return is a enum an catches OK and Err
            let pc = self.registers[Register::PC as usize];
            let instruction = self.memory.read2(pc).unwrap();
            self.registers[Register::PC as usize] = pc +2;
            println!("{} @ {}", instruction, pc);
            Ok(())
       
        


    }
}
