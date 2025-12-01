use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");
    let mut input = String::new();
    println!("Digite alguma coisa");
    let _ = io::stdin().read_line(&mut input)
        //.expect("Falha ao ler a linha")
        ;
    //println!("Você digitou: {}", input);
    println!("Você digitou: {input}");

    let numero_secreto = rand::thread_rng().gen_range(1..=100);

    println!("O número secreto é {numero_secreto}");
}