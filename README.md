# rust-study
Repositório para estudo da linguagem Rust

## Rust
Rust é uma linguagem de programação multiparadigma compilada desenvolvida pela [Mozilla Research](https://www.mozilla.org). É projetada para ser "segura, concorrente e prática", suportando os estilos puramente funcional, procedural, e orientado a objetos. Possui suporte nativo ao WebAssembly.

Uma linguagem de programação para sistemas open-source, projetada para ajudar desenvolvedores a criarem aplicações rápidas e seguras, que queiram aproveitar ao máximo as poderosas funcionalidades dos processadores modernos multi-cores. Ela previne falhas de segmentação e garante a segurança nas threads, tudo através de uma sintaxe fácil de aprender.

Além disso, Rust oferece custo zero nas suas abstrações, mudanças de semânticas, garantia de segurança de memória, threads sem condições de corrida, generics baseados em traços, pattern matching (casamento de padrões), inferência de tipos e binding eficiente para C, com o menor tamanho em tempo de execução.  
 
## Sumario
Implementações de alguns algoritmos para estudo da sintaxe e funcionamento do Rust, segundo a [documentação oficial](https://doc.rust-lang.org/book/):

1. [Hello, World!](hello-rust/src/main.rs): O primeiro código possível 
2. [Guessing Game](guessing_game/src/main.rs): Geração de números aleatórios e utilização de condicionais e estruturas de repetição
3. [Fibonacci (usando recursão)](fibonacci/src/main.rs): Utilização de funções recursivas
4. [Fibonacci (usando abordagem gulosa)](greedy_fibonacci/src/main.rs): Utilização de vetores 
5. [Quicksort](sorting_vector/src/sorting/quicksort.rs): Utilização de [crates](https://doc.rust-lang.org/1.30.0/book/first-edition/crates-and-modules.html) 
6. [Mergesort](sorting_vector/src/sorting/mergesort.rs): Implentação do algoritmo de mesclagem com crates
7. [Struct e sintaxe de método](syntax_method/src/main.rs): Implementação de structs e sintaxe de métodos
8. [Enum e pattern matching](defining_enum/src/main.rs): Implementação de enums do tipo Option<T> e casamento de padrões com o ```match```
