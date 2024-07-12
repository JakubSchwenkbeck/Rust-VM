
/** Building a 16Bit Virtual Machine in Rust, with help of https://www.youtube.com/watch?v=oArXOAhzOdY
 *  Goals: Firstly learn Rust syntax and concepts, then might really do a VM in the end:)
 */

use crate::memory::{LinMem,Addressable};

enum Register{
    A,B,C, M, SP, PC, BP, FLAGS,  // Registers like Stack pointer, programmcounter etc
} 



pub struct Machine{ 
     registers: [u16; 8] , // array of our registers
     memory : Box<dyn Addressable>, // first version of memory Box puts it on the heap instead of stack, might change later
}

impl Machine{ // creates a machine
    pub fn new() -> Self { // 
        Self{
            registers:[0;8],
            memory: Box::new(LinMem::new(8 * 1024)),
        }
    }
    pub fn step(&mut self) { // } -> Result<(),&'static str> { // Return is a enum an catches OK and Err
            let pc = self.registers[Register::PC as usize];
            let instruction = self.memory.read2(pc).unwrap();
            println!("{} @ {}",instruction,pc);
          //      OK({});


    }

}