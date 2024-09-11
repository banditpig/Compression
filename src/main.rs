use serde::{Deserialize, Serialize};
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct HuffmanNode {
    frequency: usize,
    ch: Option<char>,
    left_node: Option<Box<HuffmanNode>>,
    right_node: Option<Box<HuffmanNode>>,
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency) // reverse order for min-heap
    }
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
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

#[derive(Serialize, Deserialize, Debug, Default)]
struct BinaryData {
    original_length: usize,
    table: HashMap<char, usize>,
    encoded_data: Vec<u8>,
}

impl BinaryData {
    fn new(original_length: usize, table: HashMap<char, usize>, encoded_data: Vec<u8>) -> Self {
        BinaryData {
            original_length,
            table,
            encoded_data,
        }
    }
}

fn make_nodes(freqs: &HashMap<char, usize>) -> BinaryHeap<HuffmanNode> {
    let n: BinaryHeap<HuffmanNode> = freqs
        .iter()
        .map(|(c, f)| HuffmanNode::new(*f, Some(*c)))
        .collect();

    //dbg!(&n);
    n
}

fn frequencies(chs: &str) -> HashMap<char, usize> {
    let mut freq: HashMap<char, usize> = HashMap::new();
    chs.chars().for_each(|ch| {
        *freq.entry(ch).or_insert(0) += 1;
    });
    freq
}

fn make_huffman_tree(nodes: &mut BinaryHeap<HuffmanNode>) -> HuffmanNode {
    while nodes.len() > 1 {
        let left = nodes.pop().expect("Heap should have at least one node");
        let right = nodes.pop().expect("Heap should have at least one node");
        let mut new_node = HuffmanNode::new(left.frequency + right.frequency, None);
        new_node.left_node = Some(Box::new(left));
        new_node.right_node = Some(Box::new(right));
        nodes.push(new_node);
    }
    nodes.pop().unwrap() //.expect("Heap should have at least one node")
}

fn make_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
    if let Some(character) = node.ch {
        codes.insert(character, prefix);
        dbg!(codes);
        return;
    }
    if let Some(left) = &node.left_node {
        make_codes(left, format!("{}0", prefix), codes);
    }
    if let Some(right) = &node.right_node {
        make_codes(right, format!("{}1", prefix), codes);
    }
}

fn encode(codes: &HashMap<char, String>, data: &str) -> String {
    data.chars()
        .map(|ch| codes[&ch].as_str())
        .collect::<String>()
}

fn decode(bin_string: &str, root: &HuffmanNode) -> String {
    let mut text = String::new();
    let mut current = root;
    for ch in bin_string.chars() {
        current = if ch == '0' {
            current.left_node.as_ref().unwrap()
        } else {
            current.right_node.as_ref().unwrap()
        };
        if current.right_node.is_none() && current.left_node.is_none() {
            text.push(current.ch.unwrap());
            current = root;
        }
    }
    text
}
fn bin_string_to_bytes_vec(bin_string: &str) -> Vec<u8> {
    let padded_len = (bin_string.len() + 7) / 8 * 8;
    let padded_str = format!("{:0<width$}", bin_string, width = padded_len);
    dbg!(&padded_str);
    let v = padded_str
        .as_bytes()
        .chunks(8)
        .map(|chunk| {
            let byte_str = std::str::from_utf8(chunk).unwrap();
            u8::from_str_radix(byte_str, 2).unwrap()
        })
        .collect();
    dbg!(&v);
    v
}

fn bytes_vec_to_bin_string(bytes: Vec<u8>) -> String {
    let mut bin_string = String::new();
    for b in bytes.iter().rev() {
        for ix in 0..8 {
            let bit = (b >> ix) & 1;
            bin_string.push(if bit == 0 { '0' } else { '1' });
        }
    }
    bin_string.chars().rev().collect()
}

fn write_to_file(encoded: String, freqs: HashMap<char, usize>, fname: &str) -> std::io::Result<()> {
    let all_bytes = bin_string_to_bytes_vec(&encoded);
    let bin_data = BinaryData::new(encoded.len(), freqs, all_bytes);
    let encoded: Vec<u8> = bincode::serialize(&bin_data).unwrap();
    let mut file = File::create(fname)?;
    file.write_all(&encoded)?;
    Ok(())
}

fn read_from_file(fname: &str) -> std::io::Result<BinaryData> {
    let file = File::open(fname)?;
    let bd: BinaryData = bincode::deserialize_from(file).unwrap();
    Ok(bd)
}

fn _read_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
fn encrypt(data: &str) -> BinaryData {
    let freqs = frequencies(data);
    let mut nodes = make_nodes(&freqs);
    let tree = make_huffman_tree(&mut nodes);
    let mut codes = HashMap::new();
    make_codes(&tree, String::new(), &mut codes);
    let bin_string = encode(&codes, data);
    dbg!(&bin_string);
    let v = bin_string_to_bytes_vec(&bin_string);
    let b = BinaryData::new(data.len(), freqs, v);
    b
}
fn decrypt(data: &BinaryData) -> String {
    let mut nodes = make_nodes(&data.table);
    let tree = make_huffman_tree(&mut nodes);
    let bin_string = bytes_vec_to_bin_string(data.encoded_data.clone());
    dbg!(&bin_string);
    let mut res = decode(&bin_string, &tree);
    res.truncate(data.original_length);
    res
}

fn main() -> std::io::Result<()> {
    let data = "hello"; // read_file("path/to/file.txt")?;
    let freqs = frequencies(&data);
    let mut nodes = make_nodes(&freqs);
    //let mut ok = true;
    // while (ok) {
    //     dbg!(&nodes.pop());
    //     if (nodes.is_empty()) {
    //         ok = false;
    //     }
    // }
    let tree = make_huffman_tree(&mut nodes);
    dbg!(&tree);
    let mut codes = HashMap::new();
    make_codes(&tree, String::new(), &mut codes);
    let encoded = encode(&codes, &data);
    let _decoded = decode(&encoded, &tree);
    let all_bytes = bin_string_to_bytes_vec(&encoded);
    let _back = bytes_vec_to_bin_string(all_bytes);
    write_to_file(encoded, freqs, "test.bin")?;
    read_from_file("test.bin")?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encrypt_decrypt1() {
        let data = "hello";
        let bd = encrypt(data);
        let new_data = decrypt(&bd);
        assert_eq!(data, new_data);
    }

    #[test]
    fn encrypt_decrypt2() {
        let data = "hellohellohello how are you doing today?, I'm well thank you";
        let bd = encrypt(data);
        let new_data = decrypt(&bd);
        assert_eq!(data, new_data);
    }
    #[test]
    fn encrypt_decrypt3() {
        let data = "hellohellohello how are you doing today?, I'm well thank you, hellohellohello how are you doing today?, I'm well thank you, sdhsdsdsdshdsdsjm763£££$$%^BHjfshdfshfsfskjsjk";
        let bd = encrypt(data);
        let new_data = decrypt(&bd);
        assert_eq!(data, new_data);
    }
    #[test]
    fn test_frequencies() {
        let data = "hello";
        let freqs = frequencies(data);
        let mut expected = HashMap::new();
        expected.insert('h', 1);
        expected.insert('e', 1);
        expected.insert('l', 2);
        expected.insert('o', 1);
        assert_eq!(freqs, expected);
    }

    #[test]
    fn test_make_nodes() {
        let mut freqs = HashMap::new();
        freqs.insert('a', 5);
        freqs.insert('b', 9);
        freqs.insert('c', 12);
        let nodes = make_nodes(&freqs);
        assert_eq!(nodes.len(), 3);
    }

    #[test]
    fn test_make_huffman_tree() {
        let mut freqs = HashMap::new();
        freqs.insert('a', 5);
        freqs.insert('b', 9);
        freqs.insert('c', 12);
        let mut nodes = make_nodes(&freqs);
        let tree = make_huffman_tree(&mut nodes);
        assert_eq!(tree.frequency, 26);
    }

    #[test]
    fn test_make_codes() {
        let mut freqs = HashMap::new();
        freqs.insert('a', 5);
        freqs.insert('b', 9);
        freqs.insert('c', 12);
        let mut nodes = make_nodes(&freqs);
        let tree = make_huffman_tree(&mut nodes);
        let mut codes = HashMap::new();
        make_codes(&tree, String::new(), &mut codes);
        assert!(codes.contains_key(&'a'));
        assert!(codes.contains_key(&'b'));
        assert!(codes.contains_key(&'c'));
    }
    #[test]
    fn test_encode_more() {
        let data = "Hello, World!";
        let freqs = frequencies(data);
        let mut huff = make_nodes(&freqs);
        let huff_tree = make_huffman_tree(&mut huff);
        let mut codes = HashMap::new();
        make_codes(&huff_tree, String::new(), &mut codes);
        let encoded = encode(&codes, data);

        let decoded = decode(&encoded, &huff_tree);

        assert_eq!(decoded, data);
    }
    #[test]
    fn test_encode() {
        let mut codes = HashMap::new();
        codes.insert('a', "0".to_string());
        codes.insert('b', "10".to_string());
        codes.insert('c', "11".to_string());
        let data = "abc";
        let encoded = encode(&codes, data);
        assert_eq!(encoded, "01011");
    }

    #[test]
    fn test_decode() {
        let mut freqs = HashMap::new();
        freqs.insert('a', 5);
        freqs.insert('b', 9);
        freqs.insert('c', 12);
        let mut nodes = make_nodes(&freqs);
        let tree = make_huffman_tree(&mut nodes);
        let mut codes = HashMap::new();
        make_codes(&tree, String::new(), &mut codes);
        let encoded = encode(&codes, "abc");
        let decoded = decode(&encoded, &tree);
        assert_eq!(decoded, "abc");
    }

    #[test]
    fn test_bin_string_to_bytes_vec() {
        let bin_string = "01000001";
        let bytes = bin_string_to_bytes_vec(bin_string);
        assert_eq!(bytes, vec![65]);
    }

    #[test]
    fn test_bytes_vec_to_bin_string() {
        let bytes = vec![65];
        let bin_string = bytes_vec_to_bin_string(bytes);
        assert_eq!(bin_string, "01000001");
    }
}
