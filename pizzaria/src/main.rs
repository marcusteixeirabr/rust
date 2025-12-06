use std::io;
use clearscreen::clear;

enum Tamanho {
    Pequena,
    Média,
    Grande,
}

enum Sabor {
    Mussarela,
    Calabresa,
    Marguerita,
    Portuguesa,
}

enum StatusPedido {
    Criado,
    EmPreparo,
    Pronto,
    Entregue,
}

struct Pedido {
    numero: u32,
    tamanho: Tamanho,
    sabor: Sabor,
    status: StatusPedido,
}

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
            _ => {
                println!("\nSeu pedido já foi entregue!");
                StatusPedido::Entregue},
        }
    }

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
        };
    
        format!("Pizza {} de {} -- Status: {}", tamanho_str, sabor_str, status_str)
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
fn menu(titulo: &str, escolhas: &[&str]) -> u8 {
    let linha = "-".repeat(40);
    loop {
        clear().expect("Erro ao limpar a tela");

        println!("{}", linha);
        println!("{:^40}", "Rust's Pizza!");
        println!("{}", linha);
        println!("{:^40}", titulo);
        println!("{}", linha);

        for (i, escolha) in escolhas.iter().enumerate() {
            println!("[{}] - {}", i + 1, escolha);
        }

        if escolhas.is_empty() {
            return 0;
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


fn main() {

    let mut sequncial_pedidos: u32 = 1;

    let mut pedidos: Vec<Pedido> = Vec::new();

    loop {
        let opcao = menu("Menu Principal", &[
            "Criar um novo pedido",
            "Consultar pedidos",
            "Atualizar status do pedido",
            "Cancelar pedido",
            "Sair",
        ]);
    
        match opcao {
            1 => {
                // Lógica para criar um novo pedido
                let tamanho = menu("Escolha o tamanho da pizza", &[
                    "Pequena",
                    "Média",
                    "Grande",
                    "Voltar ao menu principal",
                ]);
                if tamanho == 4 {
                    continue;
                }
                let sabor = menu("Escolha o sabor da pizza", &[
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
                    sequncial_pedidos,
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
                sequncial_pedidos += 1;
            },
            2 => {
                // Lógica para consultar pedidos
            },
            3 => {
                // Lógica para atualizar status do pedido
            },
            4 => {
                // Lógica para cancelar pedido
            },
            5 => {
                println!("Saindo do sistema. Obrigado por usar Rust's Pizza!");
            },
            _ => unreachable!(),
        }
        
        println!("Você escolheu a opção: {}", opcao);

        if !pedidos.is_empty() {
            println!("{}", pedidos[0].descricao());
        }

    }

}
