pub fn run_with(nsets: u32,
                bsize: u32,
                assoc: u32,
                repl: super::ReplacementPolicy ,
                verbosity: i32,
                input_file: String) -> () {
    let nbits_indice = log_2(nsets);
    let nbits_offset = log_2(bsize);
    let tamanho_instrucao = 32;
    let nbits_tag = tamanho_instrucao - nbits_indice - nbits_offset;
}

// De @ExpHP em https://users.rust-lang.org/t/logarithm-of-integers/8506/4
// Passar 0 vai resultar em underflow.
// A conta não vai dar errado porque ja foi testado que o número é potência de 2.
fn log_2(x: u32) -> u32 {
    std::mem::size_of::<u32>() as u32 - x.leading_zeros() - 1
}
