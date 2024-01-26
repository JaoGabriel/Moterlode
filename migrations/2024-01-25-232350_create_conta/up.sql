CREATE TABLE Conta (
  id uuid PRIMARY KEY,
  nome VARCHAR NOT NULL,
  saldo DECIMAL(16) NOT NULL,
  usuario_id uuid NOT NULL REFERENCES Usuario (id),
  icone VARCHAR NOT NULL
);