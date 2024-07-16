fn main() {


} 
pub struct ShiftRegister{
    degree : u8,
    polynoms : [bool; 8]
          
}
impl ShiftRegister{
    pub fn new(&self, deg : u8) -> Self { // 
        Self{
            degree : deg,
            polynoms : [false ; 8],
       
        }
    }

   
   

    pub fn step_register(&mut self,_input : bool)-> bool{


-
            self.polynoms[0] = _input ;


       true 
    }

} 

