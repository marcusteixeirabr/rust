// pizzaria/src/main.rs
// Sistema de pedidos para uma pizzaria em Rust
// Autor: marcusteixeirabr
// Data: 2024-06-10
// Licença: MIT
// Versão: 0.1.0
// Descrição: Este programa simula um sistema de pedidos para uma pizzaria, permitindo criar, consultar, atualizar e cancelar pedidos de pizza.
// Uso: Execute o programa e siga as instruções no menu para interagir com o sistema de pedidos.
// Dependências: clearscreen = "1.0"
use std::io;
use std::thread;
use std::mem;
use std::time::Duration;
use clearscreen::clear;
// Enumeração para os tamanhos de pizza
enum Tamanho {
    Pequena,
    Média,
    Grande,
}
// Enumeração para os sabores de pizza
enum Sabor {
    Mussarela,
    Calabresa,
    Marguerita,
    Portuguesa,
}
// Enumeração para o status do pedido
enum StatusPedido {
    Criado,
    EmPreparo,
    Pronto,
    Entregue,
    Cancelado,
}
// Estrutura que representa um pedido de pizza
struct Pedido {
    numero: u32,
    tamanho: Tamanho,
    sabor: Sabor,
    status: StatusPedido,
}
// Implementação dos métodos para a struct Pedido
impl Pedido {
    fn novo(numero: u32, tamanho: Tamanho, sabor: Sabor) -> Self {
        println!("Seu pedido foi criado com sucesso!");
        Pedido {
            numero,
            tamanho,
            sabor,
            status: StatusPedido::Criado,
         }
    }
    // Atualiza o status do pedido
    fn atualizar(&mut self) {
        self.status = match self.status {
            StatusPedido::Criado => {
                println!("\nSeu pedido está sendo preparado!");
                StatusPedido::EmPreparo},
            StatusPedido::EmPreparo => {
                println!("\nSeu pedido está pronto!");
                StatusPedido::Pronto},
            StatusPedido::Pronto => {
                println!("\nSeu pedido foi entregue!");
                StatusPedido::Entregue},
            StatusPedido::Entregue => {
                println!("\nSeu pedido já foi entregue!");
                StatusPedido::Entregue},
            StatusPedido::Cancelado => {
                println!("\nSeu pedido foi cancelado!");
                StatusPedido::Cancelado},
        }
    }
    // Retorna uma descrição do pedido
    fn descricao(&self) -> String {
        let tamanho_str = match self.tamanho {
            Tamanho::Pequena => "Pequena",
            Tamanho::Média => "Média",
            Tamanho::Grande => "Grande",
        };
    
        let sabor_str = match self.sabor {
            Sabor::Mussarela => "Mussarela",
            Sabor::Calabresa => "Calabresa",
            Sabor::Marguerita => "Marguerita",
            Sabor::Portuguesa => "Portuguesa",
        };
    
        let status_str = match self.status {
            StatusPedido::Criado => "Criado",
            StatusPedido::EmPreparo => "Em Preparo",
            StatusPedido::Pronto => "Pronto",
            StatusPedido::Entregue => "Entregue",
            StatusPedido::Cancelado => "Cancelado",
        };
    
        format!("Pedido nº {}: Pizza {} de {} -- Status: {}", self.numero, tamanho_str, sabor_str, status_str)
    }
}

/// Apresenta o menu e retorna a opção escolhida
/// # Arguments
/// * `titulo` - Título do menu
/// * `escolhas` - Opções disponíveis no menu
/// # Returns
/// * `u8` - Opção escolhida pelo usuário
/// # Examples
/// ```
/// let opcao = menu("Menu Principal", &["Opção 1", "Opção 2", "Sair"]);
/// ``` 
fn menu(titulo: &str, escolhas: &Vec<&str>) -> u8 {
    let linha = "-".repeat(70);
    loop {
        clear().expect("Erro ao limpar a tela");

        println!("{}", linha);
        println!("{:^70}", "Rust's Pizza!");
        println!("{}", linha);
        println!("{:^70}", titulo);
        println!("{}", linha);

        if escolhas.is_empty() {
            return 0;
        }
        
        for (i, escolha) in escolhas.iter().enumerate() {
            println!("[{}] - {}", i + 1, escolha);
        }

        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada)
        .expect("Erro na entrada de dados pelo teclado");
        if let Ok(opcao) = entrada.trim().parse::<u8>() {
            if opcao >= 1 && opcao <= escolhas.len() as u8 {
                return opcao;
            }
        }
    }
}

fn consultar_pedidos(proposito: &str, pedidos: &mut Vec<Pedido>) {
    if pedidos.is_empty() {
        menu(proposito, &vec![]);
        println!("{:^70}", "Nenhum pedido encontrado");
        println!("\nPressione Enter para voltar ao Menu Principal...");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        return;
    }

    let pedidos_str: Vec<String> = pedidos.iter()
        .map(|p| p.descricao())
        .collect();
    
    let mut pedidos_str_refs: Vec<&str> = pedidos_str.iter()
        .map(|s| s.as_str())
        .collect();

    pedidos_str_refs.push("Voltar ao Menu Principal");

    let selecao = menu(proposito, &pedidos_str_refs);
    
    if selecao as usize == pedidos_str_refs.len() {
        println!("\nVoltando ao Menu Principal...");
        return;
    }
    
    let pedido_selecionado = &mut pedidos[(
        selecao - 1) as usize];

    if proposito == "Atualizar Pedido" {
        pedido_selecionado.atualizar();
    } else if proposito == "Cancelar Pedido" {
        pedido_selecionado.status = StatusPedido::Cancelado;
        println!("\nO pedido nº {} foi cancelado com sucesso!", pedido_selecionado.numero);
    }



}

fn main() {

    let mut sequencial_pedidos: u32 = 1;
    let mut pedidos: Vec<Pedido> = Vec::new();

    loop {
        let opcao = menu("Menu Principal", &vec![
            "Criar um novo pedido",
            "Atualizar status do pedido",
            "Cancelar pedido",
            "Sair",
        ]);
    
        match opcao {
            1 => {
                // Lógica para criar um novo pedido
                let tamanho = menu("Escolha o tamanho da pizza", &vec![
                    "Pequena",
                    "Média",
                    "Grande",
                    "Voltar ao menu principal",
                ]);
                if tamanho == 4 {
                    continue;
                }
                let sabor = menu("Escolha o sabor da pizza", &vec![
                    "Mussarela",
                    "Calabresa",
                    "Marguerita",
                    "Portuguesa",
                    "Voltar ao menu principal",
                ]);
                if sabor == 5 {
                    continue;
                }
                pedidos.push(Pedido::novo(
                    sequencial_pedidos,
                    match tamanho {
                        1 => Tamanho::Pequena,
                        2 => Tamanho::Média,
                        _ => Tamanho::Grande,
                    },
                    match sabor {
                        1 => Sabor::Mussarela,
                        2 => Sabor::Calabresa,
                        3 => Sabor::Marguerita,
                        _ => Sabor::Portuguesa,
                    },
                ));
                sequencial_pedidos += 1;
            },
            2 => consultar_pedidos("Atualizar Pedido", &mut pedidos),
            3 => consultar_pedidos("Cancelar Pedido", &mut pedidos),
            4 => {
                println!("\nSaindo do sistema. Obrigado por usar Rust's Pizza!\n");
                break;
            },
            _ => unreachable!(),
        }
        // Pausa antes de voltar ao menu
        thread::sleep(Duration::from_secs(2));      
    }
}
