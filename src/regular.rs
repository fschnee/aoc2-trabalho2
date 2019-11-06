pub fn run_with(nsets: usize,
                bsize: usize,
                assoc: usize,
                repl: super::cache::ReplacementPolicy ,
                verbosity: u8,
                input_file: String) -> () {
    let nbits_indice = log_2(nsets);
    let nbits_offset = log_2(bsize);
    let tamanho_instrucao = 32;
    let nbits_tag = tamanho_instrucao - nbits_indice - nbits_offset;
}

// De @ExpHP em https://users.rust-lang.org/t/logarithm-of-integers/8506/4
// Passar 0 vai resultar em underflow.
// A conta não vai dar errado porque ja foi testado que o número é potência de 2.
fn log_2(x: usize) -> usize {
    std::mem::size_of::<usize>() - x.leading_zeros() as usize - 1
}
