
/** Building a 16Bit Virtual Machine in Rust, with help of https://www.youtube.com/watch?v=oArXOAhzOdY
 *  Goals: Firstly learn Rust syntax and concepts, then might really do a VM in the end:)
 */

use crate::memory::{LinMem,Addressable};
use std::fmt;

// Define the Register enum with all its variants
pub enum Register {
    A,
    B,
    C,
    D,
    Arg1,
    Arg2,
    Val1,
    Val2,
    Temp1,
    Temp2,
    Temp3,
    Temp4,
    SP,      // Stack Pointer
    PC,      // Program Counter
    BP,      // Base Pointer
    FLAGS,   // Flags register
}

// Implement Debug trait for Register
impl fmt::Debug for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::A => write!(f, "Register A"),
            Register::B => write!(f, "Register B"),
            Register::C => write!(f, "Register C"),
            Register::D => write!(f, "Register D"),
            Register::Arg1 => write!(f, "Argument 1"),
            Register::Arg2 => write!(f, "Argument 2"),
            Register::Val1 => write!(f, "Value 1"),
            Register::Val2 => write!(f, "Value 2"),
            Register::Temp1 => write!(f, "Temp Register 1"),
            Register::Temp2 => write!(f, "Temp Register 2"),
            Register::Temp3 => write!(f, "Temp Register 3"),
            Register::Temp4 => write!(f, "Temp Register 4"),
            Register::SP => write!(f, "Stack Pointer"),
            Register::PC => write!(f, "Program Counter"),
            Register::BP => write!(f, "Base Pointer"),
            Register::FLAGS => write!(f, "Flags Register"),
        }
    }
}

// Your existing code continues below...

pub trait EnumIter {
     fn iter() -> std::slice::Iter<'static, Register>;
}

// Implement the EnumIter trait for Register enum
 impl EnumIter for Register {
    fn iter() -> std::slice::Iter<'static, Register> {
        static REGISTERS: [Register; 16] = [
            Register::A,
            Register::B,
            Register::C,
            Register::D,
            Register::Arg1,
            Register::Arg2,
            Register::Val1,
            Register::Val2,
            Register::Temp1,
            Register::Temp2,
            Register::Temp3,
            Register::Temp4,
            Register::SP,
            Register::PC,
            Register::BP,
            Register::FLAGS,
        ];
        REGISTERS.iter()
    }
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
          /*  
            
            self.registers[Register::PC as usize] = pc +2; */
            let pc = self.registers[Register::PC as usize];
            let instruction = self.memory.read2(pc ).unwrap();
            println!("{} @ {}", instruction, pc);
          
            Ok(())
         
       
    }
}
