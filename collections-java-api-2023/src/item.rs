// Em Rust, definimos a estrutura dos dados (equivalente a atributos privados da classe Java)
#[derive(Debug, Clone)] // O 'Debug' permite imprimir com {:?} e 'Clone' permite copiar o Item
pub struct Item {
    pub nome: String,
    pub preco: f64,
    pub quantidade: u32, // u32 para garantir que a quantidade seja positiva
}

// O bloco 'impl' contém os métodos da struct (equivalente aos métodos da classe Java)
impl Item {
    // Construtor (métodos construtores são geralmente chamados 'new' em Rust)
    // Usamos 'String' para receber a propriedade do nome e evitar cópias.
    pub fn new(nome: String, preco: f64, quantidade: u32) -> Item {
        Item {
            nome,
            preco,
            quantidade,
        }
    }

    // Método para calcular o subtotal. Não precisa ser 'pub' se for usado apenas internamente
    pub fn calcular_subtotal(&self) -> f64 {
        self.preco * (self.quantidade as f64)
    }
    
}