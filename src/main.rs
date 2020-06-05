mod cache;
mod helper;

use std::io;

fn main() {
    let message = helper::process::commands();
    println!("{}", message);
    
    let mut lfu_cache = cache::LFU::LFUCache::new(0);
    loop {
        let mut input = String::new();
        io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");
        helper::process::handle_input(&mut lfu_cache, &mut input)
    }
    
}
