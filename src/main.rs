pub mod myheritage;

fn main() {
    let d = myheritage::myheritage_loader(19);
    if d.is_ok() {
        println!("{:#?}", d);
    }
    
}
