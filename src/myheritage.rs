use csv::Reader;
use serde::{Serialize, Deserialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct SegmentData {
    id: String,
    matchname: String,
    chr: u8,
    start: u32,
    end: u32,
    cm: f32,
    snps: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Segments {
    data_vector: Vec<SegmentData>,
}

impl Segments {
    fn add(&mut self, data: SegmentData) {
        self.data_vector.push(data);
    }
}

pub fn myheritage_loader (selected_chromosome: u8) -> Result<String, serde_json::Error> {
    let file_path = "data/tore-mh.csv";
    let result = Reader::from_path(file_path);
    
    if result.is_err() {
        println!("Unable to read csv!");
    }

    let mut myreader = result.unwrap();

    let mut mysegments = Segments{data_vector: Vec::<SegmentData>::new()};
    
    for record in myreader.records() {
        let data = record.unwrap();
        let myid = data.get(0).unwrap().to_string();
        let myname = data.get(2).unwrap().to_string();
        let num: u8 = data.get(3).unwrap().parse().expect("not a number");
        let mystart: u32 = data.get(4).unwrap().parse().expect("not a number");
        let myend: u32 = data.get(5).unwrap().parse().expect("not a number");
        let mycm: f32 = data.get(8).unwrap().parse().expect("not a number");
        let mysnps: u32 = data.get(9).unwrap().parse().expect("not a number");
        if num == selected_chromosome {
            let mydata = SegmentData{id: myid, matchname: myname, chr: num, start: mystart, end: myend, cm: mycm, snps: mysnps};
            mysegments.add(mydata);
        }        
    }
    let myjson = to_string(&mysegments);
    return myjson;

}