#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tamanho {
    Pequena,
    Média,
    Grande,
}

impl Tamanho {
    pub fn as_str(&self) -> &'static str {
        match self {
            Tamanho::Pequena => "Pequena",
            Tamanho::Média => "Média",
            Tamanho::Grande => "Grande",
        }
    }
}

impl std::fmt::Display for Tamanho {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
