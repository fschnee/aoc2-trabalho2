extern crate clap;

fn main() {
    let yaml = clap::load_yaml!("cli.yml");
    let app = clap::App::from_yaml(yaml);
    let matches = app.get_matches();

    // No caso de argumento em falta o clap automaticamente imprime uma
    // mensagem de uso dizendo o que ta faltando, porém precisamos cuidar
    // se o argumento é do tipo certo nos unwrap() e value_t_or_exit!()
    if let Some(_x) = matches.subcommand_matches("full") {
        println!("Ainda não implementado, volte mais tarde");
        //csimlib::full(args);
    }
    else {
        use csimlib::TryPowerOfTwo;
        let nsets = clap::value_t_or_exit!(matches.value_of("nsets"), usize)
                    .try_power_of_two()
                    .map_err(|num|{format!("Malformed argument <nsets>: '{}' is not a power of 2", num)})
                    .unwrap_or_else(|err|{eprintln!("{}", err); std::process::exit(1);});
        let bsize = clap::value_t_or_exit!(matches.value_of("bsize"), usize)
                    .try_power_of_two()
                    .map_err(|num|{format!("Malformed argument <bsize>: '{}' is not a power of 2", num)})
                    .unwrap_or_else(|err|{eprintln!("{}", err); std::process::exit(1);});
        let assoc = clap::value_t_or_exit!(matches.value_of("nsets"), usize)
                    .try_power_of_two()
                    .map_err(|num|{format!("Malformed argument <assoc>: '{}' is not a power of 2", num)})
                    .unwrap_or_else(|err|{eprintln!("{}", err); std::process::exit(1);});

        let repl = match matches.value_of("repl").unwrap().to_ascii_lowercase().as_ref() {
            "l" | "lru"    => {csimlib::cache::ReplacementPolicy::Lru},
            "f" | "fifo"   => {csimlib::cache::ReplacementPolicy::Fifo},
            "r" | "random" => {csimlib::cache::ReplacementPolicy::Random},
            // Esse caso não deveria acontecer, no cli.yml tem os
            // possíveis valores para esse argumento.
            _  => {panic!();}
        };
        let verbosity = clap::value_t_or_exit!(matches.value_of("verbosity"), u8);
        let input_file = matches.value_of("input_file").unwrap().to_owned();

        csimlib::regular::run_with(nsets, bsize, assoc, repl, verbosity, input_file);
    }
}
