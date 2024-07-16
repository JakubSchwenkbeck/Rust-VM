// Virtual Machine Binary 
use rust_projects::*;
use rust_projects::instructions_lin_mem::*;


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
