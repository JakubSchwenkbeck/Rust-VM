// Virtual Machine Binary 
use rust_projects::vm::Machine;

pub fn main()-> Result<(), & 'static str> {
    let mut vm = Machine::new();
    vm.step()?;
    Ok(())


}
