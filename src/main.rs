extern crate clap;
extern crate rand;

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let app = clap::App::from_yaml(yaml);
    let matches = app.get_matches();

    // No caso de argumento em falta o clap automaticamente imprime uma
    // mensagem de uso dizendo o que ta faltando, porém precisamos cuidar
    // se o argumento é do tipo certo nos unwrap() e value_t_or_exit!()
    if let Some(_x) = matches.subcommand_matches("full") {
        unimplemented!();
    } else if let Some(submatches) = matches.subcommand_matches("regular_random") {
        use rand::RngCore;

        let input = (
            submatches.value_of("inputsize").unwrap(),
            submatches
                .value_of("seed")
                .map(|s| s.to_owned())
                .unwrap_or_else(|| rand::thread_rng().next_u64().to_string()),
        );

        let params = csimlib::regular::parse_and_validate(
            submatches.value_of("nsets").unwrap(),
            submatches.value_of("bsize").unwrap(),
            submatches.value_of("assoc").unwrap(),
            submatches.value_of("repl").unwrap(),
            submatches.value_of("verbosity").unwrap(),
            csimlib::Either::Right(input),
        )
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

        csimlib::regular::run_with(&params).print_perf(params.verbosity);
    } else {
        let params = csimlib::regular::parse_and_validate(
            matches.value_of("nsets").unwrap(),
            matches.value_of("bsize").unwrap(),
            matches.value_of("assoc").unwrap(),
            matches.value_of("repl").unwrap(),
            matches.value_of("verbosity").unwrap(),
            csimlib::Either::Left(matches.value_of("input_file").unwrap()),
        )
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });

        csimlib::regular::run_with(&params).print_perf(params.verbosity);
    };
}
