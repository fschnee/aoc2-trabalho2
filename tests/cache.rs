#[test]
fn cache_create_test() {
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

#[test]
#[ignore]
// Só deve rodar se os arquivos estiverem presente.
// Testes baseados nos exemplos dados (mandados por email).
fn control_tests() {
    let args_list = vec![
        //vec![nsets, bsize, assoc, repl, verbosity, input]
        vec!["256", "4", "1", "R", "1", "testfiles/bin_100.bin"],
        vec!["256", "4", "1", "R", "1", "testfiles/bin_1000.bin"],
        vec!["256", "4", "1", "R", "1", "testfiles/bin_10000.bin"],
        vec!["64", "4", "1", "R", "1", "testfiles/bin_100.bin"],
        vec!["64", "4", "1", "R", "1", "testfiles/bin_1000.bin"],
        vec!["64", "4", "1", "R", "1", "testfiles/bin_10000.bin"],
        vec!["32", "8", "1", "R", "1", "testfiles/bin_100.bin"],
        vec!["32", "8", "1", "R", "1", "testfiles/bin_1000.bin"],
        vec!["32", "8", "1", "R", "1", "testfiles/bin_10000.bin"],
    ];

    for args in args_list {
        let params = csimlib::parse_and_validate(
            args[0],
            args[1],
            args[2],
            args[3],
            args[4],
            csimlib::Either::Left(args[5]),
        )
        .unwrap();
        csimlib::run_with(&params).print_perf(params.verbosity);
    }
}
