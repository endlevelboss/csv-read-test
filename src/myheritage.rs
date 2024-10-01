use csv::Reader;
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
}


pub fn load_segments_of (file_path: &str, id: &str, selected_chromosome: u32, my_segment_map: &mut HashMap<String, Vec<SegmentData>>) {
    // let file_path = "data/".to_string() + id + ".csv";

    println!("reading file: {}", file_path);
    let result = Reader::from_path(file_path);
    
    if result.is_err() {
        println!("Unable to read csv!");
        return;
    }

    let mut myreader = result.unwrap();

    

    
    for record in myreader.records() {
        let data = record.unwrap();
        print!("{:?}", data);
        let myid = data.get(1).unwrap().to_string();
        let myname = data.get(2).unwrap().to_string();
        let num: u32 = data.get(2).unwrap().parse().expect("not a number");
        let mystart: u32 = data.get(3).unwrap().parse().expect("not a number");
        let myend: u32 = data.get(4).unwrap().parse().expect("not a number");
        let mycm: f32 = data.get(7).unwrap().parse().expect("not a number");
        let mysnps: u32 = data.get(8).unwrap().parse().expect("not a number");
        if num == selected_chromosome {
            let mydata = SegmentData{id: myid, chr: num, start: mystart, end: myend, cm: mycm, snps: mysnps};
            // mysegments.push(mydata);
            my_segment_map.entry(id.to_string())
                .or_insert_with(Vec::new)
                .push(mydata);
        }        
    }
    
    // my_segment_map.insert(id.to_string(), mysegments);

}

pub fn segment_loader (selected: u32) {
    let id1 = "tore";
    let mut mydata: HashMap<String, Vec<SegmentData>> = HashMap::new();
    mydata.insert(id1.to_string(), Vec::<SegmentData>::new());
    
    let file_dir1 = "data/";
    // let file_dir2 = "data/".to_string() + id2 + "/";

    for entry in fs::read_dir(file_dir1).unwrap() {
        let path = entry.unwrap().path();
        load_segments_of(path.to_str().unwrap(), id1, selected, &mut mydata);
    }
           

    // load_segments_of(id1, selected, &mut mydata);

    // load_segments_of(id2, selected, &mut mydata);

    println!("loaded stuff");
    

    
}