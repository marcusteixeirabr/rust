use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {

    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");
    println!("     JOGO DA ADIVINHAÇÃO");
    println!("-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-");

    println!("Adivinhe o número!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("O número secreto é: {secret_number}");

    loop {
        println!("Por favor digite seu palpite. ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Falha ao ler a entrada");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Você precisa digitar um número!");
                continue
            }
        };

        println!("Seu palpite foi: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Muito abaixo!"),
            Ordering::Greater => println!("Muito acima!"),
            Ordering::Equal => {
                println!("Você adivinhou!");
                break;
            }
        }
    }
}
