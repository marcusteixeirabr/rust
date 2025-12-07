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
use std::time::Duration;
use clearscreen::clear;
// Enumeração para os tamanhos de pizza
#[derive(Debug, Clone, Copy, PartialEq)]
enum Tamanho {
    Pequena,
    Média,
    Grande,
}
impl Tamanho {
    fn as_str(&self) -> &'static str {
        match self {
            Tamanho::Pequena => "Pequena",
            Tamanho::Média => "Média",
            Tamanho::Grande => "Grande",
        }
    }
}

// Enumeração para os sabores de pizza
#[derive(Debug, Clone, Copy, PartialEq)]
enum Sabor {
    Mussarela,
    Calabresa,
    Marguerita,
    Portuguesa,
}
impl Sabor {
    fn as_str(&self) -> &'static str {
        match self {
            Sabor::Mussarela => "Mussarela",
            Sabor::Calabresa => "Calabresa",
            Sabor::Marguerita => "Marguerita",
            Sabor::Portuguesa => "Portuguesa",
        }
    }
}

// Enumeração para o status do pedido
#[derive(Debug, Clone, Copy, PartialEq)]
enum StatusPedido {
    Criado,
    EmPreparo,
    Pronto,
    Entregue,
    Cancelado,
}
impl StatusPedido {
    fn as_str(&self) -> &'static str {
        match self {
        StatusPedido::Criado => "Criado",
        StatusPedido::EmPreparo => "Em Preparo",
        StatusPedido::Pronto => "Pronto",
        StatusPedido::Entregue => "Entregue",
        StatusPedido::Cancelado => "Cancelado",
        }
    }
}

// Estrutura que representa um pedido de pizza
#[derive(Debug, Clone, Copy, PartialEq)]
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
    fn atualizar_status(&mut self) {
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
        format!("Pedido nº {}: Pizza {} de {} -- Status: {}",
        self.numero,
        self.tamanho.as_str(),
        self.sabor.as_str(),
        self.status.as_str())
    }


    fn criar_pedido(id: u32) -> Option<Pedido> {
    let tamanho = menu("Escolha o tamanho da pizza", &[
        Tamanho::Pequena.as_str(),
        Tamanho::Média.as_str(),
        Tamanho::Grande.as_str(),
        "Voltar ao menu principal",
    ]);

    if tamanho == 4 {
        return None;
    }

    let sabor = menu("Escolha o sabor da pizza", &[
        Sabor::Mussarela.as_str(),
        Sabor::Calabresa.as_str(),
        Sabor::Marguerita.as_str(),
        Sabor::Portuguesa.as_str(),
        "Voltar ao menu principal",
    ]);

    if sabor == 5 {
        return None;
    }

    Some(Pedido::novo(
        id,
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
    ))
    }


    fn listar_pedidos(pedidos: &Vec<Pedido>) -> Vec<String> {

        let mut lista_pedidos: Vec<String> = pedidos.iter()
            .map(|p| p.descricao())
            .collect();
        lista_pedidos.push(String::from("Voltar ao Menu Principal"));

        return lista_pedidos;
    }


    fn selecionar_pedidos(proposito: &str, pedidos: Vec<String>) -> usize {
        
        if pedidos.is_empty() {
            menu("Lista de Pedidos", &[]);
            println!("{:^70}", "Nenhum pedido encontrado");
            println!("\nPressione Enter para voltar ao Menu Principal...");
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).unwrap();
            return 0;
        }
        
        let opcoes: Vec<&str> = pedidos.iter().map(|s| s.as_str()).collect();
        let selecao = menu(proposito, &opcoes);
        
        if selecao == pedidos.len() {
            println!("\nVoltando ao Menu Principal...");
            return 0;
        }
        return selecao;
    }


    fn atualizar_pedido(pedidos: &mut Vec<Pedido>) {
        let selecao = Pedido::selecionar_pedidos(
            "Atualizar Pedido",
            Pedido::listar_pedidos(&pedidos));

        let pedido_selecionado = &mut pedidos[selecao - 1];
        
        if pedido_selecionado.pode_atualizar() {
            pedido_selecionado.atualizar_status();
        } else {
            println!("\nNão é possível atualizar um pedido cancelado ou entregue!");
        }
    }


    fn cancelar_pedido(pedidos: &mut Vec<Pedido>) {
        let selecao = Pedido::selecionar_pedidos(
            "Cancelar Pedido",
             Pedido::listar_pedidos(&pedidos));

        let pedido_selecionado = &mut pedidos[selecao - 1];

        pedido_selecionado.status = StatusPedido::Cancelado;
        println!("\nO pedido nº {} foi cancelado com sucesso!", pedido_selecionado.numero);
    }
    

    fn pode_atualizar(&self) -> bool {
        !matches!(self.status, StatusPedido::Entregue | StatusPedido::Cancelado)
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
fn menu(titulo: &str, escolhas: &[&str]) -> usize {
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
        if let Ok(opcao) = entrada.trim().parse::<usize>() {
            if opcao >= 1 && opcao <= escolhas.len() {
                return opcao;
            }
        }
    }
}


fn main() {

    let mut sequencial_pedidos: u32 = 1;
    let mut pedidos: Vec<Pedido> = Vec::new();

    loop {
        let opcao = menu("Menu Principal", &[
            "Criar um novo pedido",
            "Atualizar status do pedido",
            "Cancelar pedido",
            "Sair",
        ]);
    
        match opcao {
            // Criar um novo pedido
            1 => {
                if let Some(p) = Pedido::criar_pedido(sequencial_pedidos) {
                    pedidos.push(p);
                    sequencial_pedidos += 1;
                }
            },
            // Atualizar status do pedido
            2 => Pedido::atualizar_pedido(&mut pedidos),
            // Cancelar pedido"Cancelar Pedido", &mut pedidos),
            3 => Pedido::cancelar_pedido(&mut pedidos),
            // Sair
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
