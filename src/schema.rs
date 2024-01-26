// @generated automatically by Diesel CLI.

diesel::table! {
    categoria (id) {
        id -> Uuid,
        nome -> Varchar,
        logo -> Varchar,
    }
}

diesel::table! {
    conta (id) {
        id -> Uuid,
        nome -> Varchar,
        saldo -> Decimal,
        usuario_id -> Uuid,
        icone -> Varchar,
    }
}

diesel::table! {
    ganho (id) {
        id -> Uuid,
        nome -> Varchar,
        valor -> Decimal,
        data -> Timestamp,
        categoria_id -> Uuid,
        conta_id -> Uuid,
        recorrente -> Nullable<Bool>,
        frequencia -> Varchar,
    }
}

diesel::table! {
    gasto (id) {
        id -> Uuid,
        nome -> Varchar,
        valor -> Decimal,
        data -> Timestamp,
        categoria_id -> Uuid,
        conta_id -> Uuid,
        recorrente -> Nullable<Bool>,
        frequencia -> Varchar,
    }
}

diesel::table! {
    investimento (id) {
        id -> Uuid,
        nome -> Varchar,
        valor -> Decimal,
        data -> Timestamp,
        categoria_id -> Uuid,
        conta_id -> Uuid,
        recorrente -> Nullable<Bool>,
        frequencia -> Varchar,
    }
}

diesel::table! {
    usuario (id) {
        id -> Uuid,
        nome -> Varchar,
        email -> Varchar,
        senha -> Varchar,
    }
}

diesel::joinable!(conta -> usuario (usuario_id));
diesel::joinable!(ganho -> categoria (categoria_id));
diesel::joinable!(ganho -> conta (conta_id));
diesel::joinable!(gasto -> categoria (categoria_id));
diesel::joinable!(gasto -> conta (conta_id));
diesel::joinable!(investimento -> categoria (categoria_id));
diesel::joinable!(investimento -> conta (conta_id));

diesel::allow_tables_to_appear_in_same_query!(
    categoria,
    conta,
    ganho,
    gasto,
    investimento,
    usuario,
);
