
use crate::memory::{LinMem,Addressable};
use crate::vm::Register;

use crate::vm::Machine;
pub enum InstructionSet{

        // Register (might be in linear memory)

    Lw, // load Word || Input : <Destination : Addr> <Source : Addr> 
    LwI, // load Word || Input :  <Destination : Addr> <_input : Word>

    SW, // store word ||input : <Destination : Addr>, <_Source : Addr> 
    SWI, // store word ||input : <Destination : Addr>, <_input : Word>
   
    Add, // Add words ||input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    AddI, // Add words immediate ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    Sub, // sub words ||input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    SubI, // sub words immediate |input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    And, //  Logical And || input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    AndI, //  Logical And ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    Or,//  Logical Or || input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    OrI, //  Logical Or ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    Jump, //Jump || input : <Destination : Addr>

    Bne, // Branch (if not equal) || <Comp1 : Addr>, <Comp2 : Addr>, <Jump : Addr>

    Slt, // set less than || <comp1 : Addr>, <Comp2 : Addr>, <AddressToSet>

    ShiftLeft, // shift (amount) || <Destination>, <Source :Addr>, <ShiftAmount : Word>
    ShiftRight, // shift (amount) || <Destination>, <Source :Addr>, <ShiftAmount : Word>

    
}

pub fn load_word( mach :&mut Machine ,dest: u16,source : u16) {
 let  val =  mach.memory.read(source);
 if val.is_some(){
    mach.memory.write(dest, val.unwrap());
 }


}
pub fn load_word_immediate(mach :&mut Machine ,dest: u16,_input :  u8) {
    
        mach.memory.write(dest, _input);
     
    
}
pub fn store_word(mach :&mut Machine ,dest: u16,source : u16){
    let  val =  mach.memory.read(source);
    if val.is_some(){
       mach.memory.write(dest, val.unwrap());
    }
   
}
pub fn store_word_immediate(mach :&mut Machine ,dest: u16,_input :  u8) {
    mach.memory.write(dest, _input);
    
}
pub fn add(mach :&mut Machine ,dest: u16,source1 : u16, source2 : u16){
    let  val1 =  mach.memory.read(source1);
    let  val2 =  mach.memory.read(source2);
    
    if val1.is_some() && val2.is_some(){
            let val = val1.unwrap() + val2.unwrap();
       mach.memory.write(dest,val);
    }
   
}
pub fn add_immediate(mach :&mut Machine ,dest: u16,source : u16,_input :  u8) {
    let  val1 =  mach.memory.read(source);

    if val1.is_some(){
            let val = val1.unwrap() + _input;
       mach.memory.write(dest,val);
    }
    
}
pub fn sub(mach :&mut Machine ,dest: u16,source1 : u16, source2 : u16){
    let  val1 =  mach.memory.read(source1);
    let  val2 =  mach.memory.read(source2);
    
    if val1.is_some() && val2.is_some(){
            let val = val1.unwrap() - val2.unwrap();
       mach.memory.write(dest,val);
    }
}

pub fn sub_immediate(mach :&mut Machine ,dest: u16,source : u16,_input :  u8) {
    let  val1 =  mach.memory.read(source);

    if val1.is_some(){
            let val = val1.unwrap() - _input;
       mach.memory.write(dest,val);
    }
    
}
pub fn and(mach :&mut Machine ,dest: u16,source1 : u16, source2 : u16){
    let  val1 =  mach.memory.read(source1);
    let  val2 =  mach.memory.read(source2);
    
    if val1.is_some() && val2.is_some(){
            let val = val1.unwrap() & val2.unwrap();
       mach.memory.write(dest,val);
    }
}
pub fn and_immediate(mach :&mut Machine ,dest: u16,source : u16,_input :  u8) {
    let  val1 =  mach.memory.read(source);

    if val1.is_some(){
            let val = val1.unwrap() & _input;
       mach.memory.write(dest,val);
    }
    
}
pub fn or(mach :&mut Machine ,dest: u16,source1 : u16, source2 : u16){
    let  val1 =  mach.memory.read(source1);
    let  val2 =  mach.memory.read(source2);
    
    if val1.is_some() && val2.is_some(){
            let val = val1.unwrap() | val2.unwrap();
       mach.memory.write(dest,val);
    }
}
pub fn or_immediate(mach :&mut Machine ,dest: u16,source : u16,_input : u8) {

    let  val1 =  mach.memory.read(source);

    if val1.is_some(){
            let val = val1.unwrap() | _input;
       mach.memory.write(dest,val);
    }
}
pub fn jump(mach :&mut Machine ,dest: u16){
  //   let pc = mach.registers[Register::PC as usize];
    mach.registers[Register::PC as usize] = dest -2;   // because it will be stepped afterwards
}

pub fn branch_not_equal(mach :&mut Machine ,comp1 : u16,comp2 : u16,jump_addr: u16){
    if comp1 == comp2 {
    jump(mach, jump_addr);
    }

}
pub fn set_less_than(mach :&mut Machine ,comp1 : u16,comp2 : u16,dest: u16){
if comp1 < comp2{
    mach.memory.write(dest, 1);
}

}
pub fn shift_left(mach :&mut Machine ,dest: u16, source:u16, shift_amount : u8){
    let  val1 =  mach.memory.read(source);

    if val1.is_some(){
            let val = val1.unwrap() << shift_amount;
       mach.memory.write(dest,val);
    }

}
pub fn shift_right(mach :&mut Machine ,dest: u16, source:u16, shift_amount : u8){
    let  val1 =  mach.memory.read(source);

    if val1.is_some(){
            let val = val1.unwrap() >> shift_amount;
       mach.memory.write(dest,val);
    }
}