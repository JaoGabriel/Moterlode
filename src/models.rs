use chrono::NaiveDateTime;
use diesel::{deserialize::Queryable, prelude::*, sql_types::Decimal};
use uuid::Uuid;
use crate::schema::*;

pub enum Frequencia{
    Nunca,
    Diario,
    Semanal,
    Mensal,
    Anual,
}

#[derive(Queryable)]
pub struct Categoria{
    pub id: Uuid,
    pub nome: String,
    pub logo: String,
}

#[derive(Insertable)]
#[diesel(table_name = categoria)]
pub struct NewCategoria{
    pub nome: String,
    pub logo: String,
}

#[derive(Queryable)]
pub struct Conta{
    pub id: Uuid,
    pub nome: String,
    pub saldo: Decimal,
    pub icone: String,
    pub usuario: Usuario,
}

#[derive(Insertable)]
#[diesel(table_name = conta)]
pub struct NewConta{
    pub nome: String,
    pub saldo: Decimal,
    pub usuario_id: Uuid,
    pub icone: String,
}

#[derive(Queryable)]
pub struct Usuario{
    pub id: Uuid,
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub contas: Vec<Conta>,
}

#[derive(Queryable)]
pub struct Ganho{
    pub id: Uuid,
    pub nome: String,
    pub valor: Decimal,
    pub data: NaiveDateTime,
    pub categoria: Categoria,
    pub recorrente: bool,
    pub frequencia: Frequencia,
    pub conta: Conta,
}

#[derive(Queryable)]
pub struct Gasto{
    pub id: Uuid,
    pub nome: String,
    pub valor: Decimal,
    pub data: NaiveDateTime,
    pub categoria: Categoria,
    pub recorrente: bool,
    pub frequencia: Frequencia,
    pub conta: Conta,
}

#[derive(Queryable)]
pub struct Investimento{
    pub id: Uuid,
    pub nome: String,
    pub valor: Decimal,
    pub data: NaiveDateTime,
    pub categoria: Categoria,
    pub recorrente: bool,
    pub frequencia: Frequencia,
    pub conta: Conta,
}


pub struct ContaResumo{
    pub id: Uuid,
    pub nome: String,
    pub saldo: Decimal,
    pub ganhos: Vec<Ganho>,
    pub gastos: Vec<Gasto>,
    pub investimentos: Vec<Investimento>,
    pub icone: String,
}