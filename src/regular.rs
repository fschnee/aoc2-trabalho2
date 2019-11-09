pub struct RunParams {
    nsets: usize,
    bsize: usize,
    assoc: usize,
    repl: super::cache::ReplacementPolicy,
    verbosity: u8,
    input: Vec<u32>,
}

pub fn run_with(params: RunParams) -> () {
    let nbits_indice = log_2(params.nsets);
    let nbits_offset = log_2(params.bsize);
    let tamanho_instrucao = 32;
    let nbits_tag = tamanho_instrucao - nbits_indice - nbits_offset;

    let cache = super::cache::Cache::create(
        params.nsets,
        params.bsize, params.repl,
        params.assoc,
        super::cache::Kind::Both);
}

// De @ExpHP em https://users.rust-lang.org/t/logarithm-of-integers/8506/4
// Passar 0 vai resultar em underflow.
// A conta não vai dar errado porque ja foi testado que o número é potência de 2.
fn log_2(x: usize) -> usize {
    std::mem::size_of::<usize>() - x.leading_zeros() as usize - 1
}

pub fn parse_and_validate(
    nsets: &str,
    bsize: &str,
    assoc: &str,
    repl: &str,
    verbosity: &str,
    input_file: &str,
) -> Result<RunParams, String> {
    use super::TryPowerOfTwo;
    let nsets = str::parse::<usize>(nsets)
        .map_err(|_parse_err| {format!("Malformed argument <nsets>: cannot convert '{}' into usize", nsets)})?
        .try_power_of_two()
        .map_err(|num| {format!("Malformed argument <nsets>: '{}' is not a power of 2", num)})?;
    let bsize = str::parse::<usize>(bsize)
        .map_err(|_parse_err| {format!("Malformed argument <bsize>: cannot convert '{}' into usize", bsize)})?
        .try_power_of_two()
        .map_err(|num| {format!("Malformed argument <bsize>: '{}' is not a power of 2", num)})?;
    let assoc = str::parse::<usize>(assoc)
        .map_err(|_parse_err| {format!("Malformed argument <assoc>: cannot convert '{}' into usize", assoc)})?
        .try_power_of_two()
        .map_err(|num| {format!("Malformed argument <assoc>: '{}' is not a power of 2", num)})?;

    let repl = match repl
        .to_ascii_lowercase()
        .as_ref()
    {
        "l" | "lru" => {super::cache::ReplacementPolicy::Lru},
        "f" | "fifo" => {super::cache::ReplacementPolicy::Fifo},
        "r" | "random" => {super::cache::ReplacementPolicy::Random},
        // Esse caso não deveria acontecer, no cli.yml tem os
        // possíveis valores para esse argumento.
        _ => {panic!()}
    };
    let verbosity = str::parse::<u8>(verbosity)
        .map_err(|_parse_err| {format!("Malformed argument <verbosity>: cannot convert '{}' into u8", verbosity)})?;
    // TODO: validar e carregar o input_file (GABRIEL)
    let _input_file = input_file.to_owned();
    let input: Vec<u32> = vec![];

    Ok(RunParams{nsets, bsize, assoc, repl, verbosity, input})
}
