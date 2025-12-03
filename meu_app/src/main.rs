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

    let numero_secreto = rand::rng().random_range(1..=100);

    println!("O número secreto é {numero_secreto}");

    let question = true;

    if question {println!("teste true");}

    let mut sz = String::from("texto qualquer");

    // let s = &mut sz;


    // println!("O tamanho da string é {}", s.len());

    // println!("A variável s é: {}", s);

    let r1 = &mut sz;

   // println!("Posso usar a variável s novamente: {}", s);

    // println!("Agora a variável r1: {} e s: {}", r1, s);

    println!("Agora a variável r1: {}", r1);

    let mut p = String::from("hello worls");

    let word = first_word(&p);

    // println!("A primeira palavra termina em índice: {}", p[word]);

    p.clear();



}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}