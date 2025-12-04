//use std::io;
use std::io::{self, Write};
use clearscreen::clear;

#[derive(Debug, Copy, Clone)]
enum Cabeca {
    Franken,
    Zombos,
    Happy,
}

#[derive(Debug, Copy, Clone)]
enum Caracteristicas {
    Cabeça(Cabeca),
    Olhos(Familia),
    Nariz(Familia),
    Boca(Familia),
    Cabelo(Familia),
    Anexos(Familia),
}

#[derive(Debug, Copy, Clone)]
enum Familia {
    Wackus,
    Vegitas,
    Spritem,
    Nula
}

// Apresenta o menu principal do app
fn menu(titulo: &str, opcoes: &[&str], sair: bool) {
    let line = "-".repeat(40);
    println!("{line}");
    println!("{titulo:^40}");
    println!("{line}");
    for (i, opcao) in opcoes.iter().enumerate() {
        println!("[{}] - {}", i + 1, opcao);
    }
    if sair {
        println!("[0] - Sair")
    }
}

fn escolher_opcao(questao: &str, qtd_opcoes: u8) -> u8 {
    loop {
        println!("{questao}");
        let mut opcao = String::new();
        io::stdin()
            .read_line(&mut opcao)
            .expect("Erro ao ler os dados");

        match opcao.trim().parse() {
            Ok(num) => {
                if num < 0 || num > qtd_opcoes {
                    print!("Opção fora do intervalo. ");
                    continue
                } else {
                    return num;
                } 
            },
            Err(_) => {
                print!("Opção fora do intervalo. ");
                continue
            }
        };
    }
}

fn cria_monstro() -> [Option<Caracteristicas>; 6] {
    clear().expect("Erro ao limpar a tela");
    let mut partes: [Option<Caracteristicas>; 6] = [None; 6];
    menu("Escolha o tipo da cabeça",
    &["Franken", "Zombos", "Happy"],
    false);
    let opcao = escolher_opcao("Escolha uma opção", 3);
    partes[0] = match opcao {
        1 => Some(Caracteristicas::Cabeça(Cabeca::Franken)),
        2 => Some(Caracteristicas::Cabeça(Cabeca::Zombos)),
        3 => Some(Caracteristicas::Cabeça(Cabeca::Happy)),
        _ => {panic!("Opção inválida!")}
    };

    menu("Escolha a família dos olhos",
    &["Wackus", "Vegitas", "Spritem"],
    false);
    let opcao = escolher_opcao("Escolha uma opção", 3);
    partes[1] = match opcao {
        1 => Some(Caracteristicas::Olhos(Familia::Wackus)),
        2 => Some(Caracteristicas::Olhos(Familia::Vegitas)),
        3 => Some(Caracteristicas::Olhos(Familia::Spritem)),
        _ => {panic!("Opção inválida!")}
    };
    
    menu("Escolha a família do nariz",
    &["Wackus", "Vegitas", "Spritem", "Nula"],
    false);
    let opcao = escolher_opcao("Escolha uma opção", 4);
    partes[2] = match opcao {
        1 => Some(Caracteristicas::Nariz(Familia::Wackus)),
        2 => Some(Caracteristicas::Nariz(Familia::Vegitas)),
        3 => Some(Caracteristicas::Nariz(Familia::Spritem)),
        4 => Some(Caracteristicas::Nariz(Familia::Nula)),
        _ => {panic!("Opção inválida!")}
    };

    menu("Escolha a família do boca",
    &["Wackus", "Vegitas", "Spritem", "Nula"],
    false);
    let opcao = escolher_opcao("Escolha uma opção", 4);
    partes[3] = match opcao {
        1 => Some(Caracteristicas::Boca(Familia::Wackus)),
        2 => Some(Caracteristicas::Boca(Familia::Vegitas)),
        3 => Some(Caracteristicas::Boca(Familia::Spritem)),
        4 => Some(Caracteristicas::Boca(Familia::Nula)),
        _ => {panic!("Opção inválida!")}
    };

    menu("Escolha a família do cabelo",
    &["Wackus", "Vegitas", "Spritem", "Nula"],
    false);
    let opcao = escolher_opcao("Escolha uma opção", 4);
    partes[4] = match opcao {
        1 => Some(Caracteristicas::Cabelo(Familia::Wackus)),
        2 => Some(Caracteristicas::Cabelo(Familia::Vegitas)),
        3 => Some(Caracteristicas::Cabelo(Familia::Spritem)),
        4 => Some(Caracteristicas::Cabelo(Familia::Nula)),
        _ => {panic!("Opção inválida!")}
    };

    menu("Escolha a família dos anexos",
    &["Wackus", "Vegitas", "Spritem", "Nula"],
    false);
    let opcao = escolher_opcao("Escolha uma opção", 4);
    partes[5] = match opcao {
        1 => Some(Caracteristicas::Anexos(Familia::Wackus)),
        2 => Some(Caracteristicas::Anexos(Familia::Vegitas)),
        3 => Some(Caracteristicas::Anexos(Familia::Spritem)),
        4 => Some(Caracteristicas::Anexos(Familia::Nula)),
        _ => {panic!("Opção inválida!")}
    };  

    return partes;

}

fn determina_familia(partes: &[Option<Caracteristicas>; 6]) -> Familia {
    let mut count_wackus = 0u8;
    let mut count_vegitas = 0u8;
    let mut count_spritem = 0u8 ;

    for parte in partes.iter() {
        match parte {
            Some(Caracteristicas::Olhos(fam))
            | Some(Caracteristicas::Nariz(fam))
            | Some(Caracteristicas::Boca(fam))
            | Some(Caracteristicas::Cabelo(fam))
            | Some(Caracteristicas::Anexos(fam)) => {
                match fam {
                    Familia::Wackus => count_wackus += 1,
                    Familia::Vegitas => count_vegitas += 1,
                    Familia::Spritem => count_spritem += 1,
                    Familia::Nula => (),
                }
            },
            _ => (),
        }
    }

    if count_wackus > count_vegitas && count_wackus > count_spritem {
        return Familia::Wackus;
    } else if count_vegitas > count_wackus && count_vegitas > count_spritem {
        return Familia::Vegitas;
    } else if count_spritem > count_wackus && count_spritem > count_vegitas {
        return Familia::Spritem;
    } else {
        return if let Some(Caracteristicas::Olhos(fam)) = partes[1] { fam } else { Familia::Nula }; // Retorna a família dos olhos em caso de empate  
    }
}

fn main() {    
    
    loop {
        clear().expect("Erro ao limpar a tela");
        println!("\nFABRICA DE MONSTROS");
        println!("Sistema de Classificação de Monstros de Zuron\n");
        menu("MENU PRINCIPAL",
            &["Criar monstro interativamente",
            "Usar monstro exemplo (Franken Wackus)",
            "Usar monstro exemplo (Zombos Vegitas)",
            "Usar monstro exemplo (Happy Spritem)"],
            true);
        
        let opcao = escolher_opcao("Escolha uma opção: ", 4);

        let partes: [Option<Caracteristicas>; 6] = match opcao {
            0 => break,
            1 => {
                println!("Criando monstro interativamente...");
                cria_monstro()
        },
            2 => [Some(Caracteristicas::Cabeça(Cabeca::Franken)),
                  Some(Caracteristicas::Olhos(Familia::Wackus)),
                  Some(Caracteristicas::Nariz(Familia::Wackus)),
                  Some(Caracteristicas::Boca(Familia::Wackus)),
                  Some(Caracteristicas::Cabelo(Familia::Wackus)),
                  Some(Caracteristicas::Anexos(Familia::Wackus))],
            3 => [Some(Caracteristicas::Cabeça(Cabeca::Zombos)),
                  Some(Caracteristicas::Olhos(Familia::Vegitas)),
                  Some(Caracteristicas::Nariz(Familia::Vegitas)),
                  Some(Caracteristicas::Boca(Familia::Vegitas)),
                  Some(Caracteristicas::Cabelo(Familia::Vegitas)),
                  Some(Caracteristicas::Anexos(Familia::Vegitas))],
            4 => [Some(Caracteristicas::Cabeça(Cabeca::Happy)),
                  Some(Caracteristicas::Olhos(Familia::Spritem)),
                  Some(Caracteristicas::Nariz(Familia::Spritem)),
                  Some(Caracteristicas::Boca(Familia::Spritem)),
                  Some(Caracteristicas::Cabelo(Familia::Spritem)),
                  Some(Caracteristicas::Anexos(Familia::Spritem))],
            _ => {panic!("Opção inválida!")},
        };
        
        let familia = determina_familia(&partes);

        let cabeca = match partes[0] {
            Some(Caracteristicas::Cabeça(cab)) => cab,
            _ => panic!("Cabeça inválida!"),
        };

        clear().expect("Erro ao limpar a tela");

        menu("RESULTADO DA CLASSIFICAÇÃO DO MONSTRO",
            &[],
            false);

        println!("A cabeça do monstro é: {:?}", cabeca);

        println!("A família predominante do monstro é: {:?}", familia);

        println!("A classificação do monstro é: {:?} {:?}", cabeca, familia);

        print!("\nAperte uma tecla para continuar ...");
        io::stdout().flush().expect("Falha ao limpar o buffer");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Falha ao ler a entrada");

        }

    println!("Programa encerrado com sucesso!");

}
