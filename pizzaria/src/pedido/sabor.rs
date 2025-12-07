#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sabor {
    Mussarela,
    Calabresa,
    Marguerita,
    Portuguesa,
}

impl Sabor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Sabor::Mussarela => "Mussarela",
            Sabor::Calabresa => "Calabresa",
            Sabor::Marguerita => "Marguerita",
            Sabor::Portuguesa => "Portuguesa",
        }
    }
}

impl std::fmt::Display for Sabor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}