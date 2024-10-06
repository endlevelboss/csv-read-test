use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io;
use std::fs::File;
use flate2::read::GzDecoder;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct SnipHeader {
    chromosome: String,
    position: String,
}

#[derive(Debug, Eq)]
struct Snp {
    position: u32,
    value: u8,
}

impl Ord for Snp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for Snp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Snp {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

fn parse (in_string: &String, pos: usize) -> String {
    let mut splitted = in_string.split(",").nth(pos).unwrap().to_string();
    splitted.retain(|c| c != '"');
    return  splitted;
}

fn unzip (mypath: &str, headers: &mut HashSet<SnipHeader>, data: &mut HashMap<SnipHeader, String>) {
    let myfile = File::open(mypath).unwrap();
    let gz = GzDecoder::new(myfile);
    
    for line in io::BufReader::new(gz).lines().skip(1) {
        let myl = line.unwrap();
        let mypos = parse(&myl, 2);
        let mychr = parse(&myl, 1);
        let mycode = parse(&myl, 3);
        let header = SnipHeader{chromosome: mychr, position: mypos};
        let mykey = header.clone();
        headers.insert(header);
        data.insert(mykey, mycode);
    }
}

fn clean_data (data: &mut HashMap<SnipHeader, String>, diffset: &HashSet<&SnipHeader>) {
    for s in diffset{
        data.remove_entry(s);        
    }
}

fn generate_matchvalues (data1: &HashMap<SnipHeader,String>, data2: &HashMap<SnipHeader, String>, chromosome: &str) -> Vec<Snp> {
    let ch1: Vec<_> = data1.keys().filter(|x| x.chromosome == chromosome).collect();
    let mut matchvalues: Vec<Snp> = Vec::new();
    
    for myheader in ch1 {
        let pos: u32 = myheader.position.parse().unwrap();
        let d1 = data1.get(&myheader).unwrap();
        let mut d1_chars = d1.chars();
        let d11 = d1_chars.next().unwrap();
        let d12 = d1_chars.next().unwrap();
        let d2 = data2.get(&myheader).unwrap();
        let mut d2_chars = d2.chars();
        let d21 = d2_chars.next().unwrap();
        let d22 = d2_chars.next().unwrap();
        
        let mut newsnp = Snp{position: 0, value: 0};

        if d1 == d2 {
            newsnp = Snp{position: pos, value: 2};
        } else if d11 == d21 || d11 == d22 || d21 == d12{
            newsnp = Snp{position: pos, value: 1};
        } else {
            newsnp = Snp{position: pos, value: 0};
        }
        matchvalues.push(newsnp);
    }
    return matchvalues;
}

#[derive(Debug, Clone, Copy)]
struct Block {
    size: u32,
    value: u8,
    start: u32,
    end: u32,
}

fn blocks(sorted_values: &Vec<Snp>) {
    let mut count = 0;
    let mut prev = 0;
    let mut blockstart = 0;

    let mut blocks: Vec<Block> = Vec::new();

    for snp in sorted_values {
        if snp.value >= prev && snp.value > 0 {
            count += 1;
        } else {
            let good_block = if count > 3 {1} else {0};
            let myblock = Block{size: count, value: good_block, start: blockstart, end: snp.position};
            blocks.push(myblock);                
            count = 0;
            prev = snp.value;
            blockstart = snp.position;
        }
    }

    let mut blocks2: Vec<Block> = Vec::new();

    let mut currentblock = Block{size: 0, value: 0, start: 0, end: 0};
    
    for b in blocks {
        let tempblock = currentblock.clone();
        if b.value == 1 {
            if tempblock.value == 0 && b.size > 200 {
                currentblock = b;
            } else if b.size > 30 {
                currentblock.end = b.end;
            } else {
                blocks2.push(currentblock);
                currentblock = b;
            }
        } else if tempblock.value == 1 {
            blocks2.push(currentblock);
            currentblock = b;
        } else {
            currentblock = b;
        }
    }

    for b in blocks2 {
        println!("{} {} {} {}",b.size, b.value, b.start, b.end);
    }
    // println!("{:?}", blocks);
}

pub fn myzip(path1: &str, path2: &str) {
    let mut path1_data: HashMap<SnipHeader, String> = HashMap::new();
    let mut path2_data: HashMap<SnipHeader, String> = HashMap::new();
    let mut path1_headers: HashSet<SnipHeader> = HashSet::new();
    let mut path2_headers: HashSet<SnipHeader> = HashSet::new();
    unzip(path1, &mut path1_headers, &mut path1_data);
    unzip(path2, &mut path2_headers, &mut path2_data);
    
    let diff: HashSet<&SnipHeader> = path2_headers.symmetric_difference(&path1_headers).collect();

    clean_data(&mut path1_data, &diff);
    clean_data(&mut path2_data, &diff);


    let mut myvals = generate_matchvalues(&path1_data, &path2_data, "5");
    myvals.sort();

    blocks(&myvals);
    
    // println!("{:?}", myvals);
    


    // println!("Done {}, {}", path1_data.capacity(), path2_data.capacity());
    
}