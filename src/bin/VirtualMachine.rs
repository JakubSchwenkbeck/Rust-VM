// Virtual Machine Binary 
use rust_projects::vm::Machine;

pub fn main()-> Result<(), & 'static str> {
    let mut virtualm = Machine::new();
    //println!("reg {}",virtualm.registers[Register::PC]);
    virtualm.step()?;
    virtualm.step()?;
    virtualm.step()?;
    virtualm.step()


 


}
