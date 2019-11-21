pub struct RunParams {
    pub nsets: usize,
    pub bsize: usize,
    pub assoc: usize,
    pub repl: super::cache::ReplacementPolicy,
    pub verbosity: u8,
    pub input: Vec<u32>,
}

pub fn run_with(params: &RunParams) -> super::cache::Cache {
    let makemask = |toggled_bits: usize, offset: usize| -> Result<u32, String> {
        let tot_bits = std::mem::size_of::<u32>() * 8;
        match offset.cmp(&tot_bits) {
            std::cmp::Ordering::Greater => {
                Err(format!(
                    "offset recebido foi {}, que é maior que std::mem::size_of::<u32>() * 8",
                    offset
                ))
            },
            std::cmp::Ordering::Equal => {
                Ok(0)
            },
            std::cmp::Ordering::Less => {
                if toggled_bits >= tot_bits {
                    Ok(u32::max_value() << offset)
                } else {
                    Ok((2u32.pow(toggled_bits as u32) - 1) << offset)
                }
            }
        }
    };

    let nbits_offset = log_2(params.bsize);
    let offset_mask = std::num::NonZeroUsize::new(nbits_offset)
        .map(|num| makemask(num.get(), 0).unwrap())
        .unwrap_or(0);

    let nbits_index = log_2(params.nsets);
    let index_mask = std::num::NonZeroUsize::new(nbits_index)
        .map(|num| makemask(num.get(), nbits_offset).unwrap())
        .unwrap_or(0);

    let nbits_instrucao = 32;
    let nbits_tag = nbits_instrucao - nbits_index - nbits_offset;
    let tag_mask = std::num::NonZeroUsize::new(nbits_tag)
        .map(|num| makemask(num.get(), nbits_index + nbits_offset).unwrap())
        .unwrap_or(0);

    if params.verbosity != 1 {
        println!(
            // Precisa ser com {:#034b} ao invez de 32 porque o '#' adiciona '0b' ao inicio.
            "nbits_indice = {}\nindex_mask  = {:#034b}\nnbits_offset = {}\noffset_mask = {:#034b}\nnbits_tag = {}\ntag_mask    = {:#034b}",
            nbits_index, index_mask, nbits_offset, offset_mask, nbits_tag, tag_mask
        );
    }

    let mut cache = super::cache::Cache::create(
        params.nsets,
        params.bsize,
        params.repl,
        params.assoc,
        super::cache::Kind::Both,
    );

    for (iteration, adress) in params.input.iter().enumerate() {
        let offset = (adress & offset_mask) as usize;

        let unshifted_index = adress & index_mask;
        let index = (unshifted_index >> nbits_offset) as usize;

        let unshifted_tag = adress & tag_mask;
        let tag = (unshifted_tag >> (nbits_index + nbits_offset)) as usize;

        let res = cache.access_with(index, tag, offset);

        if params.verbosity == 2 {
            // Precisa ser com {:#034b} ao invez de 32 porque o '#' adiciona '0b' ao inicio.
            println!("iteration = {}, ret = {:?}", iteration, res);
            println!("adress = {0:#034b} {{{0}}}", adress);
            println!("offset = {0:#034b} {{{0}}}", offset);
            println!(
                "index  = {0:#034b} ==lshift {1} bits==> {2:#034b} {{{2}}}",
                unshifted_index, nbits_offset, index
            );
            println!(
                "tag    = {0:#034b} ==lshift {1} bits==> {2:#034b} {{{2}}}",
                unshifted_tag,
                nbits_index + nbits_offset,
                tag,
            );
            println!();
        }
    }

    cache
}

// De @ExpHP em https://users.rust-lang.org/t/logarithm-of-integers/8506/4
// Passar 0 vai resultar em underflow.
// A conta não vai dar errado porque ja foi testado que o número é potência de 2.
fn log_2(x: usize) -> usize {
    std::mem::size_of::<usize>() * 8 - x.leading_zeros() as usize - 1
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
        Err("Input file has wrong byte alignment".to_owned()
            + "(cannot convert from Vec<u8> to Vec<u32> without clipping)")?
    }

    let final_data: Vec<u32> = unsafe {
        #[allow(clippy::cast_ptr_alignment)]
        let temp = Vec::from_raw_parts(
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
#[ignore]
// Só deve rodar se os arquivos estiverem presente.
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
    // Ou uma string com o caminho para o arquivo
    // ou uma tupla com o tamanho do vetor e a seed.
    input: super::Either<&str, (&str, String)>,
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

    let input = match input {
        super::Either::Left(input_file) => readfile(input_file)?,
        super::Either::Right((vecsize, seed)) => {
            use rand::Rng;

            let vecsize = str::parse::<usize>(vecsize)
                .map_err(|_| conversion_error("inputsize", vecsize, "usize"))?;
            let seed = str::parse::<u64>(seed.as_ref())
                .map_err(|_| conversion_error("seed", seed.as_ref(), "u64"))?;

            let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(seed);
            let mut vec: Vec<u32> = Vec::with_capacity(vecsize);
            for _ in 0..vecsize {
                vec.push(rng.gen())
            }

            vec
        }
    };

    Ok(RunParams {
        nsets,
        bsize,
        assoc,
        repl,
        verbosity,
        input,
    })
}
