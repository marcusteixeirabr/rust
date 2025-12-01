use std::io;
use std::process::Command;

fn main() {

    limpa_tela();

    let mut vitorias:u8 = 0;
    let mut derrotas: u8 = 0;
    let mut empates: u8 = 0;

    println!("Vamos brincar de Pedra - Papel - e - Tesoura?\n");

    'principal: loop {

        let escolhas = [
            "Pedra",
            "Papel",
            "Tesoura"
        ];
        let mut keyboard = String::new();

        println!("Ok, vou fazer minha escolha!\n");
        let computador = rand::random_range(1..=3);

        println!("Sua vez de escolher");
        println!("[1] Pedra");
        println!("[2] Papel");
        println!("[3] Tesoura");
        println!("Digite um dos números acima: ");

        let jogador = loop {
            keyboard.clear();
            io::stdin()
                .read_line(&mut keyboard)
                .expect("Falha na entrada de dados");
            match keyboard.trim().parse::<u8>() {
                Ok(num) => {
                    if num >= 1 && num <=3 {
                        break num;
                    } else {
                        println!("Valor fora da faixa (1-3). Tente novamente.");
                    }
                }
                Err(_) => println!("Entrada inválida. Digite apenas números inteiros.")
            }
        };

        if jogador == computador {
            print!("\nEmpatou! ");
            empates += 1;
        } else {
            if jogador == 1 && computador == 3 ||
                jogador == 2 && computador == 1 ||
                jogador == 3 && computador == 2 {
                print!("\nVocê venceu! ");
                vitorias += 1;
            } else {
                print!("\nVocê perdeu! ");
                derrotas += 1;
            }
        }

        print!("Eu escolhi {}, ", escolhas[(computador - 1) as usize]);
        println!("você escolheu {}.\n", escolhas[(jogador - 1) as usize]);

        println!("-=-=- Placar -=-=-");
        println!("Jogador \t{}", vitorias);
        println!("Computador \t{}", derrotas);
        println!("Empates \t{}", empates);

        loop {
            println!("\nQuer jogar novamente? [S/N]");
            keyboard.clear();
            io::stdin()
                .read_line(&mut keyboard)
                .expect("Falha na entrada de dados");
            match keyboard.trim().to_lowercase().as_str() {
                "s" | "sim" | "y" | "yes" => {
                    println!("Vamos continuar jogando!!!\n");
                    limpa_tela();
                    continue 'principal;
                }

                "n" | "nao" | "não" | "no" | "not" => {
                    println!("\nFoi muito divertido, até logo!\n");
                    break 'principal;
                }

                _ => {
                    println!("Valor fora das opções válidas. Tente novamente.");
                    continue;
                }
            }
        }
    }
}

fn limpa_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}