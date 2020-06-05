
pub mod process {
    use crate::cache;
    use std::process::exit;

const CREATE:&str = "create";
const INSERT:&str = "insert";
const GET:&str = "get";
const SHOW:&str = "show\n";
const CLEAR:&str = "clear\n";
const EXIT:&str = "exit\n";


pub fn handle_input(lfu_cache:&mut cache::LFU::LFUCache ,input:&mut String) {
    let keywords: Vec<&str> = input.split(" ").collect();
    
    match keywords[0] {
        CREATE => {
            match keywords.get(1) {
                Some(capacity) => {
                    let cap = capacity.trim().parse().expect("not a number");
                    *lfu_cache = cache::LFU::LFUCache::new(cap);
                },
                None => println!("{}",commands())
            }
        },
        INSERT => {
            match keywords.get(1) {
                Some(key) => {
                    match keywords.get(2) {
                        Some(value) => {
                            let k:i32 = key.trim().parse().expect("not a number");
                            let v:i32 = value.trim().parse().expect("not a number");
                            lfu_cache.put(k, v)
                        },
                        None => println!("{}",commands())
                    }
                },
                None => println!("{}",commands())
            }
        },
        GET => {
            match  keywords.get(1) {
                Some(key) => {
                    let k:i32 = key.trim().parse().expect("not a number");
                    println!("{}", lfu_cache.get(k))
                },
                None => println!("{}",commands())
            }
        },
        CLEAR => {lfu_cache.clear_cache()},
        EXIT => {exit(0)},
        SHOW => {println!("{}", lfu_cache)}
        _ => {println!("{}",commands())}
    }
}

pub fn commands() -> String {
    format!(
"
create [capacity]          create LFUCache holder with given capacity
insert [key] [value]          insert value by key
get [key]                  extract value by key; returns -1 in case of wrong key
clear                      remove all values from cache
exit                       close program
")
}

}


