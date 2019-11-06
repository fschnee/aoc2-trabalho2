
#[test]
fn create() {
    use csimlib::cache;

    let nsets = 32;
    let bsize = 32;
    let repl = cache::ReplacementPolicy::Lru;
    let assoc = 4;
    let cache_type = cache::Kind::Data;

    let cache = cache::Cache::create(nsets, bsize, repl, assoc, cache_type);

    assert_eq!(cache.data.len(), nsets);
    for conj in &cache.data {
        assert_eq!(conj.len(), assoc);
    }
    assert_eq!(cache.kind, cache_type);
    assert_eq!(cache.info.bsize, bsize);
    assert_eq!(cache.info.repl, repl);
}
