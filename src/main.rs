use std::time::{ SystemTime };

fn main() {

    let now = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_secs(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    };

    let block = Block::new(1, now, vec![0;32], 10000, "hoge".to_string());

    println!("Blockchain rust {:?}", block);
}