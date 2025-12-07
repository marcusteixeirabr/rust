use std::io;
use clearscreen::clear;

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
pub(crate) fn menu(titulo: &str, escolhas: &[&str]) -> usize {
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
            println!("[{}] {}", i + 1, escolha);
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