

enum Register{
    A,B,C, M, SP, PC, BP, FLAGS,  // Registers like Stack pointer, programmcounter etc
} 

trait Addressable { // interface, if something is adressable it has to ...
    fn read(&self,addr: u16) -> Option<u8>; // option is an enum which is somethin or nothing 

    fn write(&mut self,addr: u16,value: u8) -> bool; // returns if successfull

    fn read2(&self,addr: u16) -> Option<u16>{
        if let Some(x0) = self.read(addr) { // if there is something

            if let Some(x1) = self.read(addr+1){ // we are little endian!!

              return Some( (x0 as u16) | ((x1 as u16) << 8)); // get two 8 bit addresses
            }
        };
        None // do nothing 
 
    }

    fn write2(&mut self,addr: u16,value: u16) -> bool{
        let lower  = value & 0xff ;       // mask and shift to get upper and lower 8 bits of 2 bytes
        let upper  = (value & 0xff00) >> 8;
        self.write(addr,lower as u8) && self.write(addr +1, upper as u8)
        
    }
    fn copy(&mut self, from : u16, to : u16,n : usize) -> bool{

            for i in 0..n{
                if let Some(x) = self.read(from + (i as u16)){ // check if read works 
                    if !self.write(to+(i as u16),x){
                        return false ;
                    }

                }else{
                    return false;
                }

            }
            
             true

    }

}


struct LinMem{  // our linear memory
    bytes: Vec<u8>,
    size :usize,
}

impl LinMem{ 

    pub fn new(n: usize) -> Self{
         Self { bytes: vec![0;n], // create n vectors of bytes
                size : n,
              }
    }

}

impl Addressable for LinMem{ // implement read and wirte for linear memory


    fn read(&self,addr: u16) -> Option<u8>{
            if (addr as usize) < self.size {
                 Some( self.bytes[addr as usize] );
            }
            None


    }


    fn write(&mut self,addr: u16,value: u8) -> bool{
        if (addr as usize) < self.size {
                self.bytes[addr as usize] = value;
              return true;
        }

                false
            


    }


}




struct Machine{ 
     registers: [u16; 8] , // array of our registers
     memory : dyn Addressable, // first version of memory
}

impl Machine{ // creates a machine
    pub fn new() -> Self { // 
        Self{
            registers:[0;8],
            memory:LinMem::new(5000),
        }
    }
    pub fn step(&mut self) { // } -> Result<(),&'static str> { // Return is a enum an catches OK and Err
            let pc = self.registers[Register::PC as usize];
            let instruction = self.memory.read2(pc).unwrap();
            println!("{} @ {}",instruction,pc);
          //      OK({});


    }

}