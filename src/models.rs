use uuid::Uuid;

pub enum Frequencia{
    Diario,
    Semanal,
    Mensal,
    Anual,
}

pub struct Categoria{
    pub id: Uuid,
    pub nome: String,
    pub descricao: String,
    pub logo: String,
}

pub struct Ganho{
    pub id: Uuid,
    pub nome: String,
    pub valor: f64,
    pub data: String,
    pub categoria: Categoria,
    pub recorrente: Option<bool>,
    pub frequencia: Option<Frequencia>,
}

pub struct Gasto{
    pub id: Uuid,
    pub nome: String,
    pub valor: f64,
    pub data: String,
    pub categoria: Categoria,
    pub recorrente: Option<bool>,
    pub frequencia: Option<Frequencia>,
}

pub struct Investimento{
    pub id: Uuid,
    pub nome: String,
    pub valor: f64,
    pub data: String,
    pub categoria: Categoria,
    pub recorrente: Option<bool>,
    pub frequencia: Option<Frequencia>,
}

pub struct Conta{
     pub id: Uuid,
     pub nome: String,
     pub saldo: f64,
     pub ganhos: Vec<Ganho>,
     pub gastos: Vec<Gasto>,
     pub investimentos: Vec<Investimento>,
     pub icone: String,
}