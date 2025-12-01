// Declaração dos módulos: Informa ao Rust para incluir o código dos arquivos
// 'item.rs' e 'carrinho.rs' que estão na mesma pasta 'src'.
mod item;
mod carrinho;

// Importa as structs dos módulos para o escopo principal
use carrinho::CarrinhoDeCompras;
// use item::Item; // Não precisamos do Item aqui, mas se precisasse seria assim

fn main() {
    // 1. Cria o carrinho (mutável, pois será alterado)
    let mut meu_carrinho = CarrinhoDeCompras::new();

    println!("--- Teste de Adição e Atualização ---");

    // 2. Adiciona itens
    // Em Rust, Strings são movidas, então usamos .to_string() para criar uma nova String
    meu_carrinho.adicionar_item("Notebook Gamer".to_string(), 5500.00, 1);
    meu_carrinho.adicionar_item("Mouse Sem Fio".to_string(), 120.50, 2);
    meu_carrinho.adicionar_item("Teclado Mecânico".to_string(), 350.00, 1);

    // 3. Adiciona um item que já existe (deve atualizar a quantidade)
    meu_carrinho.adicionar_item("Mouse Sem Fio".to_string(), 120.50, 1);

    // 4. Exibe e calcula total
    meu_carrinho.exibir_itens();

    println!("\n--- Teste de Remoção ---");

    // 5. Remove um item (passamos &str, a referência da string, para eficiência)
    meu_carrinho.remover_item("Teclado Mecânico");
    meu_carrinho.remover_item("Item Inexistente"); // Tentativa de remover item que não existe

    // 6. Exibe o resultado final
    meu_carrinho.exibir_itens();
}

