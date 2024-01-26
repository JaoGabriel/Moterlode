CREATE TABLE Ganho (
  id uuid PRIMARY KEY,
  nome VARCHAR NOT NULL,
  valor DECIMAL(16) NOT NULL,
  data TIMESTAMP NOT NULL, 
  categoria_id uuid NOT NULL REFERENCES Categoria (id),
  conta_id uuid NOT NULL REFERENCES Conta (id),
  recorrente bool,
  frequencia VARCHAR NOT NULL
);