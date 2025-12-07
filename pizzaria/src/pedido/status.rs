#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StatusPedido {
    Criado,
    EmPreparo,
    Pronto,
    Entregue,
    Cancelado,
}

impl StatusPedido {
    pub fn as_str(&self) -> &'static str {
        match self {
        StatusPedido::Criado => "Criado",
        StatusPedido::EmPreparo => "Em Preparo",
        StatusPedido::Pronto => "Pronto",
        StatusPedido::Entregue => "Entregue",
        StatusPedido::Cancelado => "Cancelado",
        }
    }
}

impl std::fmt::Display for StatusPedido {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}