### [Trabalho 2 - Simulador de caches][githubrepo]

Trabalho de desenvolvimento de um simulador de cache em linguagem de programação
efetuado no quarto semestre na disciplina de Arquitetura e Organização de
Computadores II (AOC2) pelos alunos [Frederico Schaun][fredericogithub] e
[Gabriel Gomes][gabrielgithub] seguindo o [piloto][pdftrabalho] fornecido.

### Instruções de Uso

Rodar o executável com os parâmetros definidos no [pdf do trabalho][pdftrabalho]
para uso normal.

Lembra-se também que é possível consultar o menu de ajuda com o comando
`<executável> -h` ou `<executável> --help` ou somente `<executável>`.

### Compilação

Para compilar o executável basta rodar `cargo build --release` na pasta raiz do
projeto que será criado o executável. Depois basta rodar com o comando
`cargo run --release -- <args>` na pasta raiz ou rodar diretamente o executável
com `<executável> <args>`.

### Dependências

- [rust][] == `1.37.0` (`rustc`, `cargo`, etc)
- [clap][] == `2.33` (Adquirido automaticamente quando é compilado o projeto)

[//]: (links)
[githubrepo]: https://github.com/fschnee/aoc2-trabalho2
[fredericogithub]: https://github.com/fschnee
[gabrielgithub]: https://github.com/gabrielgomes0
[pdftrabalho]: docs/resources/AOCII_Trabalho_Implementacao_Caches_2019_2.pdf
[rust]: https://www.rust-lang.org/
[clap]: https://clap.rs/
