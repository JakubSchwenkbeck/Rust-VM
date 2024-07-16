pub enum instruction_set{

        // Register (might be in linear memory)

    Lw, // load Word || Input : <Destination : Addr> <Source : Addr> 
    LwI, // load Word || Input :  <Destination : Addr> <_input : Word>

    SW, // store word ||input : <Destination : Addr>, <_Source : Addr> 
    SWI, // store word ||input : <Destination : Addr>, <_input : Word>
   
    add, // Add words ||input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    addI, // Add words immediate ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    sub, // sub words ||input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    subI, // sub words immediate |input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    and, //  Logical And || input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    andI, //  Logical And ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    or,//  Logical Or || input : <Destination : Addr>, <Source1 : Addr> , <Source2 : Addr>
    orI, //  Logical Or ||input : <Destination : Addr>,  <Source1 : Addr>, <_input : Word>

    jump, //Jump || input : <Destination : Addr>

    bne, // Branch (if not equal) || <Comp1 : Addr>, <Comp2 : Addr>, <Jump : Addr>

    slt, // set less than || <comp1 : Addr>, <Comp2 : Addr>, <AddressToSet>

    shift, // shift (amount) || <Destination>, <Source :Addr>, <ShiftAmount : Word>

    
}

pub fn LoadWord(Dest: u16,Source : u16) {


}
pub fn LoadWordImmediate(Dest: u16,_input : u16) {

    
}
pub fn StoreWord(Dest: u16,Source : u16){

}
pub fn StoreWordImmediate(Dest: u16,_input : u16) {

    
}