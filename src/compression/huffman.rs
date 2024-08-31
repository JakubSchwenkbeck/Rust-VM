use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::collections::BTreeMap;

/// A node in the Huffman Tree
#[derive(Debug, Eq, PartialEq)]
enum HuffmanNode {
    Internal { left: Box<HuffmanNode>, right: Box<HuffmanNode> },
    Leaf { symbol: char, frequency: usize },
}

impl HuffmanNode {
    fn frequency(&self) -> usize {
        match self {
            HuffmanNode::Internal { left, right } => left.frequency() + right.frequency(),
            HuffmanNode::Leaf { frequency, .. } => *frequency,
        }
    }
}

/// A min-heap for Huffman Tree construction
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency().cmp(&self.frequency())
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn build_huffman_tree(freqs: &HashMap<char, usize>) -> HuffmanNode {
    let mut heap = BinaryHeap::new();
    
    // Create leaf nodes and push them into the priority queue
    for (symbol, &frequency) in freqs {
        heap.push(HuffmanNode::Leaf {
            symbol: *symbol,
            frequency,
        });
    }
    
    // Combine nodes until only one node remains
    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();
        
        heap.push(HuffmanNode::Internal {
            left: Box::new(left),
            right: Box::new(right),
        });
    }
    
    heap.pop().unwrap() // The root node of the Huffman Tree
}

/// Generate a Huffman Code Table
fn generate_codes(node: &HuffmanNode, prefix: VecDeque<bool>, code_table: &mut HashMap<char, VecDeque<bool>>) {
    match node {
        HuffmanNode::Internal { left, right } => {
            let mut left_prefix = prefix.clone();
            left_prefix.push_back(false);
            generate_codes(left, left_prefix, code_table);
            
            let mut right_prefix = prefix.clone();
            right_prefix.push_back(true);
            generate_codes(right, right_prefix, code_table);
        }
        HuffmanNode::Leaf { symbol, .. } => {
            code_table.insert(*symbol, prefix);
        }
    }
}

fn encode(input: &str) -> (VecDeque<bool>, HashMap<char, VecDeque<bool>>) {
    let mut freqs = HashMap::new();
    
    // Compute frequency of each character
    for c in input.chars() {
        *freqs.entry(c).or_insert(0) += 1;
    }
    
    // Build Huffman Tree
    let root = build_huffman_tree(&freqs);
    
    // Generate Huffman Codes
    let mut code_table = HashMap::new();
    generate_codes(&root, VecDeque::new(), &mut code_table);
    
    // Encode input
    let mut encoded = VecDeque::new();
    for c in input.chars() {
        if let Some(code) = code_table.get(&c) {
            encoded.extend(code.iter().cloned());
        }
    }
    
    (encoded, code_table)
}

fn decode(encoded: &VecDeque<bool>, code_table: &HashMap<char, VecDeque<bool>>) -> String {
    let mut reverse_code_table = HashMap::new();
    
    // Create reverse code table
    for (symbol, code) in code_table {
        reverse_code_table.insert(code.clone(), *symbol);
    }
    
    let mut current_code = VecDeque::new();
    let mut decoded = String::new();
    
    for bit in encoded {
        current_code.push_back(*bit);
        
        if let Some(&symbol) = reverse_code_table.get(&current_code) {
            decoded.push(symbol);
            current_code.clear();
        }
    }
    
    decoded
}

fn main() {
    let input = "this is an example for huffman encoding";
    let (encoded, code_table) = encode(input);
    
    println!("Encoded: {:?}", encoded);
    println!("Code Table: {:?}", code_table);
    
    let decoded = decode(&encoded, &code_table);
    println!("Decoded: {}", decoded);
}
