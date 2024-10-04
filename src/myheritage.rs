use csv::{Reader, StringRecord};
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use std::{fs, collections::HashMap};

#[derive(Serialize, Deserialize, Debug)]
pub struct SegmentData {
    id: String,
    chr: u32,
    start: u32,
    end: u32,
    cm: f32,
    snps: u32,
    error: String,
}

fn new_myheritage(my_segment_map: &mut HashMap<String, Vec<SegmentData>>, selected_chromosome: u32, data: StringRecord, id: &str) {
    let myid = data.get(1).unwrap().to_string();
    let num: u32 = data.get(2).unwrap().parse().expect("not a number");
    let mystart: u32 = data.get(3).unwrap().parse().expect("not a number");
    let myend: u32 = data.get(4).unwrap().parse().expect("not a number");
    let mycm: f32 = data.get(7).unwrap().parse().expect("not a number");
    let mysnps: u32 = data.get(8).unwrap().parse().expect("not a number");
    if num == selected_chromosome {
        let mydata = SegmentData{id: myid, chr: num, start: mystart, end: myend, cm: mycm, snps: mysnps, error: "".to_string()};
        my_segment_map.entry(id.to_string())
            .or_insert_with(Vec::new)
            .push(mydata);
    }
}

fn old_myheritage(my_segment_map: &mut HashMap<String, Vec<SegmentData>>, selected_chromosome: u32, data: StringRecord, id: &str) {
    let myid = data.get(2).unwrap().to_string();
    let num: u32 = data.get(3).unwrap().parse().expect("not a number");
    let mystart: u32 = data.get(4).unwrap().parse().expect("not a number");
    let myend: u32 = data.get(5).unwrap().parse().expect("not a number");
    let mycm: f32 = data.get(8).unwrap().parse().expect("not a number");
    let mysnps: u32 = data.get(9).unwrap().parse().expect("not a number");
    if num == selected_chromosome {
        let mydata = SegmentData{id: myid, chr: num, start: mystart, end: myend, cm: mycm, snps: mysnps, error: "".to_string()};
        my_segment_map.entry(id.to_string())
            .or_insert_with(Vec::new)
            .push(mydata);
    }
}

fn ftdna(my_segment_map: &mut HashMap<String, Vec<SegmentData>>, selected_chromosome: u32, data: StringRecord, id: &str) {
    let myid = data.get(0).unwrap().to_string();
    let num = data.get(1).unwrap().trim();
    let chromo = if num == "X" {
        23
    } else {
        num.parse().expect("not a number")
    };
    let mystart: u32 = data.get(2).unwrap().parse().expect("not a number");
    let myend: u32 = data.get(3).unwrap().parse().expect("not a number");
    let mycm: f32 = data.get(4).unwrap().parse().expect("not a number");
    let mysnps: u32 = data.get(5).unwrap().parse().expect("not a number");
    if chromo == selected_chromosome {
        let mydata = SegmentData{id: myid, chr: chromo, start: mystart, end: myend, cm: mycm, snps: mysnps, error: "".to_string()};
        my_segment_map.entry(id.to_string())
            .or_insert_with(Vec::new)
            .push(mydata);
    }
}

pub fn load_segments_of (file_path: &str, id: &str, selected_chromosome: u32, my_segment_map: &mut HashMap<String, Vec<SegmentData>>) {
    // let file_path = "data/".to_string() + id + ".csv";

    let result = Reader::from_path(file_path);
    
    if result.is_err() {
        let errordata = SegmentData{id: file_path.to_string(), chr: 0, start: 0, end: 0, cm: 0.0, snps: 0, error: "Unable to read csv".to_string()};
        my_segment_map.entry(id.to_string())
                .or_insert_with(Vec::new)
                .push(errordata);
        println!("Unable to read csv!");
        return;
    }

    let mut myreader = result.unwrap();

    for record in myreader.records() {
        match record {
            Ok(data) => {
                let data_length = data.len();
                match data_length {
                    6 => ftdna(my_segment_map, selected_chromosome, data, id),
                    9 => new_myheritage(my_segment_map, selected_chromosome, data, id),
                    10 => old_myheritage(my_segment_map, selected_chromosome, data, id),
                    _other => return,
                }
            }
            Err(_error) => {
                let errordata = SegmentData{id: file_path.to_string(), chr: 0, start: 0, end: 0, cm: 0.0, snps: 0, error: "Error reading file".to_string()};
                my_segment_map.entry(id.to_string())
                    .or_insert_with(Vec::new)
                    .push(errordata);
            }
        }
    }
}

pub fn segment_loader (selected: u32) {
    let id1 = "tore";
    let id2 = "randi";
    let mut mydata: HashMap<String, Vec<SegmentData>> = HashMap::new();
    mydata.insert(id1.to_string(), Vec::<SegmentData>::new());
    
    let file_dir1 = "data/".to_string() + id1 + "/";
    let file_dir2 = "data/".to_string() + id2 + "/";

    for entry in fs::read_dir(file_dir1).unwrap() {
        let path = entry.unwrap().path();
        load_segments_of(path.to_str().unwrap(), id1, selected, &mut mydata);
    }

    for entry in fs::read_dir(file_dir2).unwrap() {
        let path = entry.unwrap().path();
        load_segments_of(path.to_str().unwrap(), id2, selected, &mut mydata);
    }

           

    // load_segments_of(id1, selected, &mut mydata);

    // load_segments_of(id2, selected, &mut mydata);    
    let finalstuff = to_string(&mydata);
    println!("{:?}", finalstuff);
    

    
}