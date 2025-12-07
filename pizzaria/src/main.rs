// pizzaria/src/main.rs
// Sistema de pedidos para uma pizzaria em Rust
// Autor: marcusteixeirabr
// Data: 2024-06-10
// Licença: MIT
// Versão: 0.1.0
// Descrição: Este programa simula um sistema de pedidos para uma pizzaria, permitindo criar, consultar, atualizar e cancelar pedidos de pizza.
// Uso: Execute o programa e siga as instruções no menu para interagir com o sistema de pedidos.
// Dependências: clearscreen = "1.0"

pub mod menu;
pub mod pedido;

use std::thread;
use std::time::Duration;
use menu::menu;
use pedido::Pedido;


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
