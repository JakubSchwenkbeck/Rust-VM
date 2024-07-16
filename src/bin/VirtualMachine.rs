// Virtual Machine Binary 
use rust_projects::vm::Machine;
use rust_projects::instruction_set::*;
use rust_projects::memory::{LinMem,Addressable};
use rust_projects::vm::Register;


pub fn main()-> Result<(), & 'static str> {
    let mut virtualm = Machine::new();
    //println!("reg {}",virtualm.registers[Register::PC]);
    add_immediate(&mut virtualm, 1, 1, 15);
    let val =virtualm.memory.read(1);
    let mut disp = 1;
    if val.is_some(){
         disp = val.unwrap();
    }
    println!("{disp}");
    virtualm.step()


 


}
