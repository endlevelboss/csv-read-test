
#[derive(Debug, Clone, Copy)]
struct Block {
    size: u32,
    value: u8,
    start: u32,
    end: u32,
}

pub fn mytest() {
    let mut blocks: Vec<Block> = Vec::new();
    let mut currentblock = Block{size: 0, value: 0, start: 0, end: 0};
    currentblock.end = 100;
    currentblock = Block{size: 0, value: 0, start: 0, end: 20};
    blocks.push(currentblock);
    currentblock.end = 5000;
    println!("{:?}", currentblock);
    println!("{:?}", blocks)
}