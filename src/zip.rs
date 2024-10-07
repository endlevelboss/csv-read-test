use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io;
use std::fs::File;
use std::cmp;
use flate2::read::GzDecoder;
use image::{self, Rgb};

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

fn blocks(sorted_values: &Vec<Snp>, chromosome: &str) {
    let chromolength:Vec<u32> = [249250621, 243199373, 198022430, 191154276, 180915260, 171115067, 159138663, 146364022, 141213431, 135534747, 135006516, 133851895, 115169878, 107349540, 102531392, 90354753, 81195210, 78081510, 59128983, 63025520, 48129895, 51304566, 155270560].to_vec();
    let mych: usize = chromosome.parse().unwrap();
    let myindex = mych - 1;
    let mychromolength = chromolength[myindex];




    let mut count = 0;
    let mut prev = 0;
    let mut blockstart:u32 = sorted_values.first().unwrap().position;

    let mut green_blocks: Vec<Block> = Vec::new();
    let mut yellow_blocks: Vec<Block> = Vec::new();


    let sorted = sorted_values;

    for snp in sorted {
        if snp.value >= prev && snp.value > 0 {
            count += 1;
        } else {
            let good_block = if count > 3 {1} else {0};
            let myblock = Block{size: count, value: good_block, start: blockstart, end: snp.position};
            yellow_blocks.push(myblock);                
            count = 0;
            prev = snp.value;
            blockstart = snp.position;
        }
    }

    for snp in sorted_values {
        if snp.value >= prev && snp.value == 2 {
            count += 1;
        } else {
            let good_block = if count > 3 {1} else {0};
            let myblock = Block{size: count, value: good_block, start: blockstart, end: snp.position};
            green_blocks.push(myblock);                
            count = 0;
            prev = snp.value;
            blockstart = snp.position;
        }
    }

    

    let mut currentblock = Block{size: 0, value: 0, start: 0, end: 0};

    let mut green_merged: Vec<Block> = Vec::new();
    
    for b in green_blocks {
        let tempblock = currentblock.clone();
        if b.value == 1 {
            if tempblock.value == 0 && b.size > 40 {
                green_merged.push(currentblock);
                currentblock = b;
            } else if tempblock.value == 1 && b.size > 10 {
                currentblock.size += b.size;
                currentblock.end = b.end;
            } else {
                green_merged.push(currentblock);
                currentblock = b;
            }
        } else if tempblock.value == 1 {
            green_merged.push(currentblock);
            currentblock = b;
        } else {
            currentblock = b;
        }
    }
    green_merged.push(currentblock);

    green_merged.retain(|x| x.size > 100);


    let mut yellow_merged: Vec<Block> = Vec::new();
    
    for b in yellow_blocks {
        let tempblock = currentblock.clone();
        if b.value == 1 {
            if tempblock.value == 0 && b.size > 40 {
                yellow_merged.push(currentblock);
                currentblock = b;
            } else if tempblock.value == 1 && b.size > 10 {
                currentblock.size += b.size;
                currentblock.end = b.end;
            } else {
                yellow_merged.push(currentblock);
                currentblock = b;
            }
        } else if tempblock.value == 1 {
            yellow_merged.push(currentblock);
            currentblock = b;
        } else {
            currentblock = b;
        }
    }
    yellow_merged.push(currentblock);

    yellow_merged.retain(|x| x.size > 100);
    // for b in blocks2 {
    //     println!("{} {} {} {}",b.size, b.value, b.start, b.end);
    // }

    let mut img = image::RgbImage::new(2000,1);

    for i in 0..2000 {
        img.put_pixel(i, 0, image::Rgb([192, 57, 43]));
    }

    for b in yellow_merged {
        let start = (f64::from(b.start) / f64::from(mychromolength) * 2000f64 ) as u32; 
        let end = (f64::from(b.end) / f64::from(mychromolength) * 2000f64) as u32;
        let capend = cmp::min(1999, end);
        
        for i in start..capend {
            img.put_pixel(i, 0, image::Rgb([244, 208, 63]));
        }    
    }

    for b in green_merged {
        let start = (f64::from(b.start) / f64::from(mychromolength) * 2000f64 ) as u32; 
        let end = (f64::from(b.end) / f64::from(mychromolength) * 2000f64) as u32;
        let capend = cmp::min(1999, end);
        
        for i in start..capend {
            img.put_pixel(i, 0, image::Rgb([39, 174, 96]));
        }    
    }
    let mypath = chromosome.to_string() + ".png";
    img.save(mypath).unwrap();
    // println!("{:?}", blocks);
}


fn generate(data1: &HashMap<SnipHeader, String>, data2: &HashMap<SnipHeader, String>, chromosome: &str) {
    let mut myvals = generate_matchvalues(&data1, &data2, &chromosome);
    myvals.sort();

    blocks(&myvals, &chromosome);
    
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

    let myvec: Vec<i32> = (1..23).collect();
    let mychromos: Vec<String> = myvec.into_iter().map(|x| x.to_string()).collect();

    for ch in mychromos {
        let mych: &str = &ch;
        generate(&path1_data, &path2_data, mych);
    }
    
    

    
}