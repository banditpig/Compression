
use priority_queue::PriorityQueue;
use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::future::pending;
use std::io::{Write, Read};
use std::io::{Stderr};
use std::sync::mpsc::channel;
use serde::{Serialize, Deserialize};
use bincode;
use bit_vec::BitVec;


#[derive(Debug, Eq, PartialEq, Hash)]
struct HuffmanNode {
    frequency: usize,
    ch: Option<char>,
    left_node:  Option<Box<HuffmanNode>>,
    right_node: Option<Box<HuffmanNode>>,
}
impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.frequency.cmp(&self.frequency) // reverse order for min-heap
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl HuffmanNode {
    fn new(frequency: usize, ch: Option<char>) -> HuffmanNode {
        HuffmanNode {
            frequency,
            ch,
            left_node: None,
            right_node: None,
        }
    }
}

#[derive(Serialize,Deserialize, Debug, Default)]
struct BinaryData {
    //need this as there will be padding if the encode string of 1s and 0s is
    //not multiple of 8.
    original_length: usize,
    table: HashMap<char, usize>,
    encoded_data: Vec<u8>,
}

impl BinaryData {
    fn new(original_length: usize, table: HashMap<char, usize>, encoded_data: Vec<u8> ) -> Self {
       BinaryData{
           original_length,
           table,
           encoded_data,
       }
    }
}

fn make_nodes(freqs: &HashMap<char, usize>) -> BinaryHeap<HuffmanNode> {

    let heap =  freqs
        .iter()
        .map(|(c, f)| HuffmanNode::new(*f, Some(*c)))
        .collect();
    heap
}
fn frequencies(chs: &str) -> HashMap<char, usize> {
    let mut freq: HashMap<char, usize> = HashMap::new();
    for ch in chs.chars() {
        *freq.entry(ch).or_insert(0) += 1;
    }
    freq
}
fn make_huffman_tree(nodes: &mut BinaryHeap<HuffmanNode>) -> HuffmanNode {
    while nodes.len() > 1 {
        let left =nodes.pop().unwrap();
        let right = nodes.pop().unwrap();
        let mut new_node = HuffmanNode::new(left.frequency + right.frequency, None);
        new_node.left_node = Some(Box::new(left));
        new_node.right_node = Some(Box::new(right));
        let f = new_node.frequency;
        nodes.push(new_node);
    }
    nodes.pop().unwrap()
}
fn make_codes(node: &HuffmanNode, prefix: String,codes: &mut HashMap<char, String>){
    dbg!(node.ch);
    if let Some(character) = node.ch {
        codes.insert(character, prefix);
        return;
    } 
        if let Some(left) = &node.left_node {
            make_codes(left, format!("{}0", prefix), codes);
        }
        if let Some(right) = &node.right_node {
            make_codes(right, format!("{}1", prefix), codes);
        }
    
}
fn encode(codes: &HashMap<char, String>, data: &str) -> String{
    let mut encoded = String::new();
    for ch in data.chars() {
        encoded.push_str(&codes[&ch]);
    }
   encoded
}
fn decode(bin_string: &str, codes: &HashMap<char, String>, root: &HuffmanNode) -> String{
    
    let mut text =  String::new();
    let mut current = root;
    for ch in bin_string.chars(){
        if ch   == '0'{
            current = current.left_node.as_ref().unwrap();
        }
        else{
            current = current.right_node.as_ref().unwrap();
        }
        if current.right_node.is_none() && current.left_node.is_none() {
          
            text.push(current.ch.unwrap());
            current = root;
        }
    }
   text
}
fn bin_string_to_bytes_vec(bin_string: &str) -> Vec<u8>{

    //pad with 0s
    let extra = bin_string.chars().count() % 8;
    let mut padded_str = bin_string.to_string();
    for _ in  0..extra{
      padded_str.push('0');
    }
    dbg!(&padded_str);
    let mut ix: usize = 0;
    let mut byte :u8 = 0;
    let mut bytes = vec![];
    for c in padded_str.chars() {
        if c == '1' {
            byte |= 1 << ix;
        }
        ix += 1;
        if ix == 8{
            bytes.push(byte);
            ix = 0;
            byte = 0;
        }
    }
    if ix != 0 {
        bytes.push(byte);
    }
   bytes
}
fn bytes_vec_to_bin_string(bytes: Vec<u8>) -> String{
    let mut bin_string = String::new();
   
    for b in bytes { 
        let mut ix: usize = 0;
        while ix < 8 {
           let bit = (b >> ix) & 1;
           dbg!(bit);
           if bit == 0{
               bin_string.push('0')
           }else {
               bin_string.push('1')
           }
           ix += 1;
       }
    }
    bin_string
}
fn write_to_file(encoded: String, freqs: HashMap<char, usize>, fname: &str){


    // Write the binary data to a file

    let all_bytes = bin_string_to_bytes_vec(&encoded);

    let bin_data = BinaryData::new(encoded.len(), freqs, all_bytes);
    let encoded: Vec<u8> = bincode::serialize(&bin_data).unwrap();
    let mut file = File::create("data.bin").unwrap();
    file.write_all(&encoded);
    //let encoded: Vec<u8> = bincode::encode_to_vec(&bin_data, config).unwrap();
    //dbg!(encoded);

}
fn read_from_file(fname: &str) -> BinaryData{
    
   let mut bd:BinaryData =  bincode::deserialize_from(File::open("data.bin").unwrap()).unwrap();
    dbg!(&bd);
   bd
}
fn read_file() -> String{
    let mut file = File::open("/Users/mikehoughton/TinderAutomation/StatesAndCities/Georgia/acworth.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    contents
    
}
fn main() {
   
    read_from_file("");
    let data = "hello";//read_file();
    let freqs = frequencies(&data);
    // dbg!(&freqs);

    let mut nodes = make_nodes(&freqs);

    // while let Some(h) = &nodes.pop() {
    //     println!("-> {:?} ", h);
    // }
    // dbg!(&nodes);
    let tree = make_huffman_tree(&mut nodes);
  
  //  dbg!(&tree);
    
    let mut codes = HashMap::new();
    make_codes(&tree,String::new(),&mut codes);
    // dbg!(&codes);
    
    let encoded = encode(&codes, &data);
    dbg!(&encoded);
   
    let decoded = decode(&encoded, &codes, &tree);
    // dbg!(&decoded);
    let all_bytes = bin_string_to_bytes_vec(&encoded);
    let back = bytes_vec_to_bin_string(all_bytes);
    dbg!(back);
    write_to_file(encoded, freqs, "test.bin");
}
