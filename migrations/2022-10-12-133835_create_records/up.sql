CREATE TABLE records (
  id VARCHAR(10) PRIMARY KEY,
  url TEXT NOT NULL,
  token VARCHAR(6),
  expired_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP NOT NULL,
  created_at TIMESTAMP NOT NULL
);
