# Table of contents (TOC)
- [Table of contents (TOC)](#table-of-contents-toc)
- [Introdução](#introduo)
- [Compilando e rodando o programa](#compilando-e-rodando-o-programa)
- [Funcionamento](#funcionamento)
	- [Command-line interface (CLI)](#command-line-interface-cli)
	- [Subcomando `regular-random`](#subcomando-regular-random)
- [Overview da implementação](#overview-da-implementao)


---


# Introdução
O simulador foi desenvolvido na linguagem *rust* pelos alunos Frederico Schaun
e Gabriel Gomes, seguindo as especificações do trabalho, adicionando também as
seguintes funcionalidades extra:

-   As políticas de substituição LRU e FIFO.
-   Gerador de números aleatórios para endereços (ver [aqui](#subcomando-regular-random)).

*obs: o projeto também está disponível no [github][1].*


# Compilando e rodando o programa
Para gerar um executável basta seguir o procedimento padrão na linguagem rust,
ou seja, no diretório principal do projeto, rodamos:

    $ cargo build [--release]

*obs: as chaves `[...]` demarcam argumentos opcionais e o símbolo `$` denota um
      comando no terminal.*

Isso irá gerar um executável na pasta `target/...` (aonde exatamente depende dos
argumentos passados). Feito isso o programa pode ser rodado com

    $ caminho-do-executável [argumentos]

ou, preferivelmente

    $ cargo run [--release] [-- argumentos]

*obs: na versão entregue do projeto o executável compilado foi deixado na
      pasta raiz previamente, de acordo com a especificação do trabalho.*

Para o resto deste relatório a forma de rodar o programa (tudo antes de
`argumentos`) será referido como `executável`.


# Funcionamento
O simulador foi escrito, como previamente dito, em [*rust*][2] (uma linguagem
moderna de *systems programming*) e usa dois pacotes (chamados *crates* no
ecossistema *rust*): [*clap*][3] e [*rand*][4], para a interface de linha de
comando e geração de números aleatórios, respectivamente.

O projeto contém 5 (cinco) arquivos de código fonte, sendo 3 (três) deles
código de aplicação, 1 (um) para teste e 1 (um) para configuração, separados da
seguinte maneira:

| Arquivo        | Função       | Conteúdo |
|:-------------- |:------------:|:-------- |
| src/cache.rs   | Aplicação    | Código definindo o funcionamento da cache
| src/csimlib.rs | Aplicação    | Código para simulação
| src/main.rs    | Aplicação    | Código do `main` e parse de argumentos
| src/cli.yml    | Configuração | Código de configuração da [*cli*](#command-line-interface-cli), usado pelo *clap*
| tests/cache.rs | Testes       | Código para testes básicos

*obs: os arquivos de código de aplicação ocasionalmente têm testes no meio, que
      são demarcados pela annotation `#[test]` antes da função.*


## Command-line interface (CLI)
O programa segue a interface requisitada no enunciado do trabalho, porém
conta com algumas ferramentas à mais devido ao *clap*. Essa seção irá detalhar
somente o funcionamento extra.

Rodando o programa das seguintes formas

    $ executável
    $ executável -h
    $ executável --help
    $ executável help

deve mostrar a seguinte mensagem de ajuda:

    cache_simulator 1.0
    Frederico S. <fbdsschaun@inf.ufpel.edu.br>, Gabriel G. <gabriel.almgom@gmail.com>
    Simula uma cachezin

    USAGE:
        cache-simulator <nsets> <bsize> <assoc> <repl> <verbosity> <input_file>
        cache-simulator <SUBCOMMAND>

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <nsets>         :usize --> Numero de conjuntos
        <bsize>         :usize --> Tamanho do bloco, em bytes
        <assoc>         :usize --> Associatividade da cache
        <repl>          :str   --> Política de substituição [possible values: l, lru, L, LRU, Lru, f, fifo, F, FIFO,
                    Fifo, r, random, R, RANDOM, Random]
        <verbosity>     :u8    --> Flag para dizer verbosidade, 1 é o modo de saida padrão, qualquer outro valor pode
                    ser usado para debugar
        <input_file>    :str   --> Caminho para o arquivo de entrada (absoluto ou relativo)

    SUBCOMMANDS:
        help              Prints this message or the help of the given subcommand(s)
        regular_random    Versão normal do simulador porém com números aleatórios para os endereços

Essa tela é gerada pelo *clap* à partir da configuração em `src/cli.yml`, ela
contém todas as informações relevantes para a interface de terminal do programa.

Vale ressaltar também que o último dessas formas de excutar o programa pode ser
usada para obter informações dos subcomandos (`help` e `regular-random`) da
seguinte forma:

    $ executável help [SUBCOMMAND]

Ou seja, são comandos válidos:

    $ executável help help
    $ executável help regular_random


## Subcomando `regular-random`
O subcomando `regular-random` popula o vetor de endereços de entrada com uma
quantidade de números aleatórios passado como argumento, usando uma seed também
passada por argumento ou uma seed aleatória quando não é passada ao programa.

A tela de ajuda desse subcomando é a seguinte:

    cache-simulator-regular_random 1.0
    Frederico S. <fbdsschaun@inf.ufpel.edu.br>
    Versão normal do simulador porém com números aleatórios para os endereços

    USAGE:
        cache-simulator regular_random <nsets> <bsize> <assoc> <repl> <verbosity> <inputsize> [seed]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    ARGS:
        <nsets>        :usize --> Numero de conjuntos
        <bsize>        :usize --> Tamanho do bloco, em bytes
        <assoc>        :usize --> Associatividade da cache
        <repl>         :str   --> Política de substituição [possible values: l, lru, L, LRU, Lru, f, fifo, F, FIFO,
                   Fifo, r, random, R, RANDOM, Random]
        <verbosity>    :u8    --> Flag para dizer verbosidade, 1 é o modo de saida padrão, qualquer outro valor pode ser
                   usado para debugar
        <inputsize>    usize --> Tamanho do vetor de input (quantidade de endereços)
        <seed>         u64   --> Seed usada para a geração do vetor de endereços

No entando, o vetor dos endereços de entrada é do tipo `u32`, ou seja, um inteiro
positivo de 32 bits, o que torna muito improvável de acontecer um hit e, por isso,
esse subcomando é util somente para testar o processamento dos endereços de
entrada (separação em *tag*, *index* e *offset*).

# Overview da implementação
De forma resumida, o programa interpreta uma cache como uma matriz de *structs*,
mais especificamente um `Vec<Vec<csimlib::cache::Data>>`. Por motivos ergonomicos
é definida uma *trait* (*interface* em Java, *typeclass* em Haskell, etc)
`Conjunto` que depois é implementada para `Vec<csimlib::cache::Data>`,
tornando possível "ver" a struct anterior como um `Vec<csimlib::cache::Conjunto>`
(ou somente `Vec<Conjunto>` para simplicidade), da seguinte maneira:

```rust
// em src/cache.rs
#[derive(Debug)]
pub struct Data {
    pub tag: usize,
    pub is_initialized: bool,
    // Higher means more likely to be replaced
    pub replaceability: usize,
}

trait Conjunto {
    fn has_tag(&self, tag: usize) -> bool;
    fn get_index_by_tag(&self, tag: usize) -> Option<usize>;
    fn uninitialized_slots(&self) -> usize;
    fn first_vacant_slot_index(&self) -> Option<usize>;
    fn insert_tag(&mut self, tag: usize, repl: ReplacementPolicy, rng: &mut rand::rngs::StdRng);
    fn get_highest_replaceability_index(&self) -> Option<usize>;
}

impl Conjunto for Vec<Data> {
    // --snip--
}
```

*obs: usaremos `// --snip--` para denotar código cortado para o propósito de
      simplificação do entendimento.*

As funções mais importantes dessa *trait* são `has_tag` e `insert_tag`, que
verificam se a tag está contida no conjunto e inserem a tag no conjunto
aplicando a política de substituição, repectivamente. Todas as outras funções
podem ser vistas como puramente auxiliares e não é produtivo para o escopo
desse relatório analisar o seu funcionamento.

Após isso é definido uma *struct* `Cache` da seguinte forma:

```rust
// em src/cache.rs
#[derive(Debug)]
pub struct Cache {
    pub kind: Kind,
    pub performance: Performance,
    pub info: Info,
    pub data: Vec<Vec<Data>>,
}
```

Os tipos `Kind`, `Perfomance` e `Info` são definidos no mesmo arquivo mas tudo
que é necessário saber sobre eles é o seguinte:

-   `Kind` refere-se ao tipo de cache (de instrução, dados ou ambas), que acabou
    não sendo usado no projeto.
-   `Performance` mantém informações de *hits*, *misses* e seus tipos, etc.
-   `Info` mantém numero de *sets*, de slots livres, o gerador aleatório usado, etc.

Após isso foram implementadas algumas funções para a cache:

```rust
impl Cache {
    pub fn create(
        nsets: usize,
        bsize: usize,
        repl: ReplacementPolicy,
        assoc: usize,
        kind: Kind,
    ) -> Cache {
        // -- snip--
    }

    pub fn create_with_seed(
        nsets: usize,
        bsize: usize,
        repl: ReplacementPolicy,
        assoc: usize,
        kind: Kind,
        random_repl_seed: u64,
    ) -> Cache {
        // --snip--
    }

    pub fn print_perf(&self, verbosity: u8) {
        // --snip--
    }

    pub fn access_with(&mut self, index: usize, tag: usize, _offset: usize) -> AccessResult {
        // --snip--
    }
}
```

Novamente só há uma função realmente importante e ela é a `acess_with`. O nome
é autoexplicativo: ela tenta acessar a cache com os parametros passados e
retorna o resultado, que é definido da seguinte forma:

```rust
// em src/cache.rs
#[derive(Debug, PartialEq)]
pub enum AccessResult {
    Hit,
    Miss(MissTypes),
}

#[derive(Debug, PartialEq)]
pub enum MissTypes {
    Compulsory,
    Capacity,
    Conflict,
}
```

Esse resultado não chega a ser utilizado no programa, mas poderia ser aproveitado,
por exemplo, para a criação de uma interface interativa, mostrando o que aconteceu
após o acesso para o usuário.

Uma observação à ser feita é o uso de enumerações algébricas, o tipo `AcessResult`
é um *enum*, sendo que um dos seus valores possíveis é um `Miss(MissTypes)`, isso
significa que `Miss` carrega uma informação extra, o tipo do miss, ou seja, é um
*enum* que contém outro *enum*. Um uso dessa *pattern* seria, por exemplo:

```rust
#[test]
fn funcao_exemplo() {
    use csimlib::cache::*;

    let mut cache = Cache::create(/* args aqui */);
    let resultado = cache.acess_with(/* args aqui */);

    match resultado {
        AcessResult::Hit => {print!("Deu hit");},
        AcessResult::Miss(miss_type) => {print!("Deu miss do tipo {}", miss_type);}
    }
}
```

Esse tipo de construto está no coração do *rust* e linguagens funcionais em geral
e é amplamente usado, principalmente quando se lida com `Option` e `Result`.

Depois disso é simples definir um loop de simulação da seguinte forma:

```rust
// em src/csimlib.rs
pub struct RunParams {
    pub nsets: usize,
    pub bsize: usize,
    pub assoc: usize,
    pub repl: cache::ReplacementPolicy,
    pub verbosity: u8,
    pub input: Vec<u32>,
}

pub fn run_with(params: &RunParams) -> cache::Cache {
    // --snip-

    let mut cache = cache::Cache::create(
        params.nsets,
        params.bsize,
        params.repl,
        params.assoc,
        cache::Kind::Both,
    );

    for (iteration, adress) in params.input.iter().enumerate() {
        let offset = (adress & offset_mask) as usize;

        let unshifted_index = adress & index_mask;
        let index = (unshifted_index >> nbits_offset) as usize;

        let unshifted_tag = adress & tag_mask;
        let tag = (unshifted_tag >> (nbits_index + nbits_offset)) as usize;

        let res = cache.access_with(index, tag, offset);
        // --snip--
    }
    // --snip--
}
```

`RunParams` é uma struct retornada do estágio de parse do programa e tudo que é
necessário saber sobre ela é que os parametros dentro dela foram todos validados
(numeros são potência de 2 e diferente de 0, arquivo foi carregado ou vetor de input
foi gerado usando o pacote *rand*, etc).

Com isso, as partes principais do funcionamento da cache foram todas abordadas,
o que falta é faltar do parse dos argumentos da linha de comando, que foge do
escopo deste documento por requerir um conhecimento externo da linguagem e,
por isso não será explicada.


[1]: (https://github.com/fschnee/aoc2-trabalho2)
[2]: (https://rust-lang.org/)
[3]: (https://clap.rs/)
[4]: (https://docs.rs/crate/rand)
