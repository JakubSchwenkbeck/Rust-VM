

enum Register{
    A,B,C, M, SP, PC, BP, FLAGS,  // Registers like Stack pointer, programmcounter etc
} 

trait addressable { // interface, if something is adressable it has to ...
    fn read(&self,addr: u16) -> Option<u8>; // option is an enum which is somethin or nothing 

    fn write(&mut self,addr: u16,value: u8) -> bool; // returns if successfull

    fn read2(&self,addr: u16) -> Option<u16>{
        if let Some(x0) = self.read(addr) { // if there is something

            if let Some(x1) = self.read(addr+1){ // we are little endian!!

              return  (x0 as u16) | (x1 as u16 << 8); // get two 8 bit addresses
            }
        };
        None // do nothing 
 
    }

    fn write2(&mut self,addr: u16,value: u16) -> bool{

        
    }

}

fn nothin(){
    println!("nth");
}



struct Machine{ 
     registers: [u16; 8] , // array of our registers
     memory : [u8, 5000], // first version of memory
}

impl Machine{ // creates a machine
    pub fn new() -> Self { // 
        Self{
            register:[0;8],
            memory:[0;5000],
        }
    }
    pub fn step(&mut self) -> Result<(),&'static str> { // Return is a enum an catches OK and Err
            let pc ;


    }

}