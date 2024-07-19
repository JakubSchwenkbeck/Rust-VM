/*
  Implement memory alloc (filename) -> bool:


        params:   - start address 
                  - end address

        Hashmap puts (filename) to (address Intervall)

        true if adresses free
        false if not free

    */
    use crate::interval::Interval;

    pub fn mem_alloc(filename : &str)-> (bool,Interval){
        // get mem1 
        if mem_check() {
                let mem1 = 0;
                let mem2 = 10;
                let mem_interval =   Interval::new(mem1,mem2);
                
            (true,mem_interval)
        }else{
            (false, Interval::new(0,0))
        }
    }

/*
    get next free space
    



*/
/*
  Implement memory release (filename or Intervall):

        releases memory
        (deletes from hashmap)

        works with filename or an given intervall
*/
/* 

  implement memory check address -> bool

        if free true
        if not free false (not able to write)
*/
pub fn mem_check() -> bool{
    true
}