pub mod tamanho;
pub mod sabor;
pub mod status;

use std::io;
use crate::menu::menu;
use crate::pedido::status::StatusPedido;
use tamanho::Tamanho;
use sabor::Sabor;



#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Pedido {
    numero: u32,
    tamanho: Tamanho,
    sabor: Sabor,
    status: StatusPedido,
}
// Implementação dos métodos para a struct Pedido
impl Pedido {
    pub(crate) fn novo(numero: u32, tamanho: Tamanho, sabor: Sabor) -> Self {
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
        self.tamanho,
        self.sabor,
        self.status)
    }


    pub fn criar_pedido(id: u32) -> Option<Pedido> {
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


    fn listar_pedidos(pedidos: &[Pedido]) -> Vec<String> {

        let mut lista: Vec<String> = pedidos.iter()
            .map(|p| p.descricao())
            .collect();
        lista.push(String::from("Voltar ao Menu Principal"));

        return lista;
    }


    fn selecionar_pedidos(proposito: &str, pedidos: &[String]) -> usize {
        
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


    pub fn atualizar_pedido(pedidos: &mut Vec<Pedido>) {
        let lista = Pedido::listar_pedidos(&pedidos);
        let selecao = Pedido::selecionar_pedidos(
            "Atualizar Pedido",
            &lista);
        
        if selecao == 0 {
            return;
        }   

        let pedido_selecionado = &mut pedidos[selecao - 1];
        
        if pedido_selecionado.pode_atualizar() {
            pedido_selecionado.atualizar_status();
        } else {
            println!("\nNão é possível atualizar um pedido cancelado ou entregue!");
        }
    }


    pub fn cancelar_pedido(pedidos: &mut Vec<Pedido>) {
        let lista = Pedido::listar_pedidos(&pedidos);
        let selecao = Pedido::selecionar_pedidos(
            "Cancelar Pedido",
             &lista);

        if selecao == 0 {
            return;
        }   

        let pedido_selecionado = &mut pedidos[selecao - 1];

        pedido_selecionado.status = StatusPedido::Cancelado;
        println!("\nO pedido nº {} foi cancelado com sucesso!", pedido_selecionado.numero);
    }
    

    fn pode_atualizar(&self) -> bool {
        !matches!(self.status, StatusPedido::Entregue | StatusPedido::Cancelado)
    }

}