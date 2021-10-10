
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE todos (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  name text NOT NULL,
  is_done bool NOT NULL
);
