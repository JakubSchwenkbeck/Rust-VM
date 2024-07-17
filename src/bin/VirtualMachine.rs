
use instructions_regs::{reg_add, reg_immediate_add, reg_load_word, reg_print, reg_store_word, reg_jump};
// Virtual Machine Binary 
use rust_projects::*;
use rust_projects::instructions_lin_mem::*;
use rust_projects::u4::U4;

pub fn main()-> Result<(), & 'static str> {
    let mut virtualm = Machine::new();
   
   add_immediate(&mut virtualm, 1, 1, 15);
    let val =virtualm.memory.read(1).unwrap();
    println!("Value in Addr 1 :{val}");
    virtualm.step()?;

    
    reg_load_word(&mut virtualm,U4::new(1) ,1 );
    
    reg_print(&mut virtualm, U4::new(1));
   
    virtualm.step()?;

    reg_add(&mut virtualm, U4::new(2), U4::new(1), U4::new(2));
    reg_print(&mut virtualm, U4::new(2));
   

    virtualm.step()?;

    reg_store_word(&mut virtualm, 2 , U4::new(2));

    let val =virtualm.memory.read(2).unwrap();
    println!("Value in Addr 2   :{val}");
    virtualm.step()?;

    reg_immediate_add(&mut virtualm, U4::new(3), U4::new(3), U4::new(7));
    reg_print(&mut virtualm, U4::new(2));
    virtualm.step()?;

    reg_jump(&mut virtualm,11, U4::new(3) );
    virtualm.step()?;

    reg_jump(&mut virtualm,12, U4::new(0) );
    virtualm.step()



}
