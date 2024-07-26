use std::collections::HashMap;
use std::sync::Mutex;
use lazy_static::lazy_static;

const CHUNK_SIZE: u16 = 16; // Define the size of each chunk

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<String, Vec<Chunk>>> = Mutex::new(HashMap::new());
    static ref FREE_LIST: Mutex<Vec<Chunk>> = Mutex::new(vec![]);
}

static mut LATEST_ADDRESS: u16 = 0;

#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    pub start: u16,
    pub end: u16,
}

impl Chunk {
    pub fn new(start: u16) -> Self {
        Self {
            start,
            end: start + CHUNK_SIZE - 1,
        }
    }
}

pub fn get_latest_addr() -> u16 {
    unsafe { LATEST_ADDRESS }
}

pub fn mem_alloc(filename: &str, size: u16) -> bool {
    let mut map = HASHMAP.lock().unwrap();
    let mut free_list = FREE_LIST.lock().unwrap();

    if map.contains_key(filename) {
        println!("This file already allocated memory space");
        return false;
    }

    let mut chunks: Vec<Chunk> = vec![];
    let mut remaining_size = size;

    while remaining_size > 0 {
        if let Some(free_chunk) = free_list.pop() {
            chunks.push(free_chunk);
            remaining_size -= CHUNK_SIZE.min(remaining_size);
        } else {
            let new_chunk = Chunk::new(unsafe { LATEST_ADDRESS });
            unsafe { LATEST_ADDRESS += CHUNK_SIZE };
            chunks.push(new_chunk);
            remaining_size -= CHUNK_SIZE.min(remaining_size);
        }
    }

    map.insert(filename.to_owned(), chunks);
    true
}

pub fn mem_release(filename: &str) {
    let mut map = HASHMAP.lock().unwrap();
    let mut free_list = FREE_LIST.lock().unwrap();

    if let Some(chunks) = map.remove(filename) {
        for chunk in chunks {
            free_list.push(chunk);
        }
        free_list.sort_by_key(|c| c.start);
        let mut merged_list = vec![];
        let mut current = free_list[0];
        for &chunk in free_list.iter().skip(1) {
            if current.end + 1 == chunk.start {
                current.end = chunk.end;
            } else {
                merged_list.push(current);
                current = chunk;
            }
        }
        merged_list.push(current);
        *free_list = merged_list;
        println!("Memory released and merged: {:?}", free_list);
    } else {
        println!("No memory allocated for this file");
    }
}

// Mock function to simulate file size
pub fn get_file_size(filename: &str) -> u16 {
    match count_non_empty_lines(filename) {
        Ok(count) => count,
        Err(e) => {
            println!("Error: {}", e);
            0
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
        .filter_map(Result::ok)
        .filter(|line| !line.trim().is_empty())
        .count();

    Ok(count as u16)
}