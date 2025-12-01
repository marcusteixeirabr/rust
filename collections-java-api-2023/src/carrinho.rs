// Importa a struct Item do módulo 'item'
use crate::item::Item;

// Definição do Carrinho de Compras
pub struct CarrinhoDeCompras {
    // Usa um Vector para a lista de itens. O Vector é mutável (mutável)
    itens: Vec<Item>,
}

impl CarrinhoDeCompras {
    // Construtor do carrinho
    pub fn new() -> Self {
        CarrinhoDeCompras {
            itens: Vec::new(),
        }
    }

    // Adiciona um item. O método recebe 'mut self' porque o vetor 'itens' será modificado.
    pub fn adicionar_item(&mut self, nome: String, preco: f64, quantidade: u32) {
        // Verifica se o item já existe
        if let Some(item_existente) = self.itens.iter_mut()
            .find(|item| item.nome.to_lowercase() == nome.to_lowercase()) {
            // Se existe, atualiza a quantidade.
            item_existente.quantidade += quantidade;
            println!("Item '{}' atualizado. Nova quantidade: {}.", nome, item_existente.quantidade);
        } else {
            // Se não existe, cria um novo e adiciona
            let novo_item = Item::new(nome.clone(), preco, quantidade);
            self.itens.push(novo_item);
            println!("Item '{}' adicionado ao carrinho.", nome);
        }
    }

    // Remove um item. Usa 'retain' para manter apenas os itens que não correspondem ao nome.
    pub fn remover_item(&mut self, nome: &str) {
        let tamanho_antes = self.itens.len();

        // Remove todos os itens cujo nome corresponda (case-insensitive)
        self.itens.retain(|item| item.nome.to_lowercase() != nome.to_lowercase() );

        if self.itens.len() < tamanho_antes {
            println!("Item '{}' removido com sucesso.", nome);
        } else {
            println!("Erro: Item '{}' não encontrado no carrinho.", nome);
        }
    }

    // Calcula o valor total. Usa 'self' por referência (&self), pois não altera o carrinho.
    pub fn calcular_valor_total(&self) -> f64 {
        // Usa o iterador do vetor e o método 'fold' para somar os subtotais
        self.itens.iter().fold(0.0, |total, item| total + item.calcular_subtotal())
    }

    // Exibe os itens.
    pub fn exibir_itens(&self) {
        println!("\n========================================================================");
        println!("|                      CARRINHO DE COMPRAS (Rust)                      |");
        println!("========================================================================");
        if self.itens.is_empty() {
            println!("| O carrinho está vazio.                              |");
        } else {
            for item in &self.itens {
                println!(
                    "| {:<20} | R$ {:>8.2} | Qtd: {:>3} | Subtotal: R$ {:>7.2} |",
                    item.nome,
                    item.preco,
                    item.quantidade,
                    item.calcular_subtotal()
                );
            }
        }
        println!("========================================================================");
        println!("| VALOR TOTAL: R$ {:>8.2}                                             |", self.calcular_valor_total());
        println!("========================================================================");
    }

}