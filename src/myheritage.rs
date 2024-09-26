use csv::Reader;


#[derive(Debug)]
pub struct Data {
    id: String,
    matchname: String,
    chr: u8,
    start: u32,
    end: u32,
    cm: f32,
    snps: u32,
}

pub fn myheritage_loader () -> Vec<Data> {
    let file_path = "data/tore-mh.csv";
    let result = Reader::from_path(file_path);
    let chromosome: u8 = 3;

    if result.is_err() {
        println!("Unable to read csv!")
    }

    let mut myreader = result.unwrap();

    let mut mydatavector: Vec<Data> = vec![];
    // let count = myreader.records().count();
    

    for record in myreader.records() {
        let data = record.unwrap();
        let myid = data.get(0).unwrap().to_string();
        let myname = data.get(2).unwrap().to_string();
        let num: u8 = data.get(3).unwrap().parse().expect("not a number");
        let mystart: u32 = data.get(4).unwrap().parse().expect("not a number");
        let myend: u32 = data.get(5).unwrap().parse().expect("not a number");
        let mycm: f32 = data.get(8).unwrap().parse().expect("not a number");
        let mysnps: u32 = data.get(9).unwrap().parse().expect("not a number");
        if num == chromosome {
            let mydata = Data{id: myid, matchname: myname, chr: num, start: mystart, end: myend, cm: mycm, snps: mysnps};
            mydatavector.push(mydata);
        }        
    }

    return mydatavector;

}