-- Your SQL goes here
CREATE TABLE records (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50) NOT NULL,
  amount FLOAT8 NOT NULL,
  amount_io VARCHAR(3) NOT NULL,
  comment TEXT NULL DEFAULT NULL,
  record_date DATE NOT NULL DEFAULT CURRENT_DATE,
  category_id INTEGER NOT NULL REFERENCES categories(id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
  is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
  is_mutable BOOLEAN NOT NULL DEFAULT TRUE,g

  foreign key (category_id) references categories(id)
)