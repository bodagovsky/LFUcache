mod lfu;


fn main() {
    let mut lfu_cache = lfu::LFU::LFUCache::new(5);
    lfu_cache.put(1, 1);
    // lfu.put(1, 2);
    // lfu.put(1, 3);
    lfu_cache.get(1);
    lfu_cache.put(2, 1);

    lfu_cache.get(2);

    // lfu.put(2, 1);
    // lfu.put(2, 1);
    // lfu.put(2, 1);

    println!("{:?}", lfu_cache)
}
