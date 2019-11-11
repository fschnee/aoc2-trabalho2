pub struct RunParams {
    nsets: usize,
    bsize: usize,
    assoc: usize,
    repl: super::cache::ReplacementPolicy,
    verbosity: u8,
    input: Vec<u32>,
}

pub fn run_with(params: RunParams) {
    let nbits_indice = log_2(params.nsets);
    let nbits_offset = log_2(params.bsize);
    let tamanho_instrucao = 32;
    let nbits_tag = tamanho_instrucao - nbits_indice - nbits_offset;

    let cache = super::cache::Cache::create(
        params.nsets,
        params.bsize,
        params.repl,
        params.assoc,
        super::cache::Kind::Both,
    );
}

// De @ExpHP em https://users.rust-lang.org/t/logarithm-of-integers/8506/4
// Passar 0 vai resultar em underflow.
// A conta não vai dar errado porque ja foi testado que o número é potência de 2.
fn log_2(x: usize) -> usize {
    std::mem::size_of::<usize>() - x.leading_zeros() as usize - 1
}

fn conversion_error(field_name: &str, value: &str, expected_type: &str) -> String {
    format!(
        "Malformed argument <{}>: cannot convert '{}' into {}",
        field_name, value, expected_type
    )
}

fn power_of_two_error(field_name: &str, value: usize) -> String {
    format!(
        "Malformed argument <{}>: '{}' is not a power of 2",
        field_name, value
    )
}

fn readfile(filename: &str) -> Result<Vec<u32>, String> {
    let raw_data: Vec<u8> = std::fs::read(filename).map_err(|e| format!("{:#?}", e))?;
    if raw_data.len() % std::mem::size_of::<u32>() != 0 {
        Err("Input file has wrong byte alignment".to_owned())?
    }

    let final_data: Vec<u32> = unsafe {
        #[allow(clippy::cast_ptr_alignment)]
        let temp: Vec<u32> = Vec::from_raw_parts(
            raw_data.as_ptr() as *mut u32,
            raw_data.len() / std::mem::size_of::<u32>(),
            raw_data.capacity() / std::mem::size_of::<u32>(),
        )
        .into_iter()
        .map(u32::from_be)
        .collect();
        std::mem::forget(raw_data);
        temp
    };

    Ok(final_data)
}

#[test]
fn readfile_test() {
    readfile("testfiles/bin_100.bin").unwrap();
    readfile("testfiles/bin_1000.bin").unwrap();
    readfile("testfiles/bin_10000.bin").unwrap();
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
        .map_err(|_| conversion_error("nsets", nsets, "usize"))?
        .try_power_of_two()
        .map_err(|num| power_of_two_error("nsets", num))?;
    let bsize = str::parse::<usize>(bsize)
        .map_err(|_| conversion_error("bsize", bsize, "usize"))?
        .try_power_of_two()
        .map_err(|num| power_of_two_error("bsize", num))?;
    let assoc = str::parse::<usize>(assoc)
        .map_err(|_| conversion_error("assoc", assoc, "usize"))?
        .try_power_of_two()
        .map_err(|num| power_of_two_error("assoc", num))?;

    let repl = match repl.to_ascii_lowercase().as_ref() {
        "l" | "lru" => super::cache::ReplacementPolicy::Lru,
        "f" | "fifo" => super::cache::ReplacementPolicy::Fifo,
        "r" | "random" => super::cache::ReplacementPolicy::Random,
        // Esse caso não deveria acontecer, no cli.yml tem os
        // possíveis valores para esse argumento.
        _ => panic!(),
    };
    let verbosity =
        str::parse::<u8>(verbosity).map_err(|_| conversion_error("verbosity", verbosity, "u8"))?;

    let input = readfile(input_file)?;

    Ok(RunParams {
        nsets,
        bsize,
        assoc,
        repl,
        verbosity,
        input,
    })
}
