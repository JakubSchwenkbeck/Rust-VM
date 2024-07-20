/*
  Implement memory alloc (filename) -> bool:


        params:   - start address 
                  - end address

        Hashmap puts (filename) to (address Intervall)

        true if adresses free
        false if not free

    
    */
    use std::collections::HashMap;
    use std::sync::Mutex;
    use lazy_static::lazy_static;
    
    lazy_static! {
        static ref HASHMAP: Mutex<HashMap<String, Interval>> = Mutex::new(HashMap::new());
    }
    
    static mut LATEST_ADRESS: i32 = 0 ;
    #[derive(Copy, Clone, Debug)]
pub struct Interval{
    pub start : u16,
    pub end : u16,

}


impl Interval{ // creates a machine
    pub fn new(s: u16,e: u16) -> Self { // 
        Self{
            start:s,
            end : e,
            
        }
    }
}

pub fn get_latest_addr() ->i32{
    unsafe { LATEST_ADRESS }
}


pub fn get_interval(filename : &str) ->(u16,u16){
    let  map = HASHMAP.lock().unwrap();
    let x = map.get(filename).unwrap();
    (x.start, x.end)
}

    pub fn mem_alloc(filename : &str)-> (bool,Interval){
        // get file size:
        let val = get_file_size(filename);
           
        let mut map = HASHMAP.lock().unwrap();
        if !map.contains_key(filename){
       
                let mem1 = (unsafe { LATEST_ADRESS }  ) as u16;
                let mem2 = 2*  (unsafe { LATEST_ADRESS }  as u16 + val ) ;
                let mem_interval =   Interval::new(mem1,mem2);
                unsafe { LATEST_ADRESS = (mem2 +1 )as i32 };
                let result =mem_interval;
               
                map.insert(filename.to_owned(),mem_interval );
                println!("start {mem1}, end{mem2}");
            (true,result)
        }
        else{
            println!("This file already allocated Memory space");
            (false, Interval::new(0,0))
            
        }
    }

    pub fn mem_alloc_direct(filename : &str,input : u16)-> (bool,Interval){
        // get mem1 
        let mut map = HASHMAP.lock().unwrap();
        if !map.contains_key(filename){
       
                let mem1 = (unsafe { LATEST_ADRESS } +1 ) as u16;
                let mem2 = input;
                let mem_interval =   Interval::new(mem1,mem2);
                unsafe { LATEST_ADRESS = mem2 as i32 };
                let result =mem_interval;
                
                map.insert(filename.to_owned(),mem_interval );
                println!("start {mem1}, end {mem2}");
            (true,result)
        }else{
            println!("This file already allocated Memory space");
            (false,Interval::new(0,0))
        }
    }

pub fn get_file_size(filename : &str) -> u16 {
    let map = HASHMAP.lock().unwrap();
    if map.contains_key(filename) {
        let val = map.get(filename).unwrap();
        val.end - val.start

    }else{
        match count_non_empty_lines(filename) {
            Ok(count) =>count,
            Err(e) => {println!("Error: {}", e); 0},
        }
    }
 
    
}

use std::fs::File;
use std::io::{self, BufRead};


fn count_non_empty_lines(filename: &str) -> io::Result<u16> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let count = reader
        .lines()
        .filter_map(Result::ok)  // Skip over lines that can't be read
        .filter(|line| !line.trim().is_empty())  // Count only non-empty lines
        .count();

    Ok(count as u16)
}


pub fn memory_release(filename : &str) {
    let _ = filename;
}

/*
  Implement memory release (filename or Intervall):

        releases memory
        (deletes from hashmap)

        works with filename or an given intervall
*/
