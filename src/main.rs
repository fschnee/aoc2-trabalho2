extern crate clap;

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let app = clap::App::from_yaml(yaml);
    let matches = app.get_matches();

    // No caso de argumento em falta o clap automaticamente imprime uma
    // mensagem de uso dizendo o que ta faltando, porém precisamos cuidar
    // se o argumento é do tipo certo nos unwrap() e value_t_or_exit!()
    if let Some(_x) = matches.subcommand_matches("full") {
        unimplemented!();
    } else {
        let params = csimlib::regular::parse_and_validate(
            matches.value_of("nsets").unwrap(),
            matches.value_of("bsize").unwrap(),
            matches.value_of("assoc").unwrap(),
            matches.value_of("repl").unwrap(),
            matches.value_of("verbosity").unwrap(),
            matches.value_of("input_file").unwrap(),
        )
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            std::process::exit(1);
        });
        csimlib::regular::run_with(params);
    }
}
