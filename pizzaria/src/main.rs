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
    tamanho: Tamanho,
    sabor: Sabor,
    status: StatusPedido,
}

impl Pedido {
    fn novo(tamanho: Tamanho, sabor: Sabor) -> Self {
        println!("Seu pedido foi criado com sucesso!");
        Pedido {
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



fn main() {
    
    let mut pedido = Pedido::novo(
        Tamanho::Grande,
        Sabor::Calabresa,
    );

    print!("{}", pedido.descricao());

    pedido.atualizar();
    
    print!("{}", pedido.descricao());
}
