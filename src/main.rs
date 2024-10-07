pub mod myheritage;
pub mod zip;
pub mod testing;

fn main() {
    // myheritage::segment_loader(19);    
    zip::myzip("tore.csv.gz", "randi.csv.gz");
    // testing::mytest();
}
