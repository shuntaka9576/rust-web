INSERT INTO
  roles(name)
VALUES
  ('Admin'),
  ('User')
ON CONFLICT DO NOTHING;

INSERT INTO
  users(name, email, password_hash, role_id)
SELECT
  'Eleazar Fig',
  'eleazar.fig@example.com',
  '$2b$12$nQntihUyqiFlV4i3O2QxM.msFL6j4./n1yrXEglIePMaj1KGV0A12',
  role_id
FROM
  roles
WHERE
  name LIKE 'Admin';

CREATE TABLE IF NOT EXISTS checkouts(
  checkout_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  book_id UUID NOT NULL UNIQUE,
  user_id UUID NOT NULL,
  checked_out_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),

  FOREIGN KEY (book_id) REFERENCES books(book_id)
      ON UPDATE CASCADE
      ON DELETE CASCADE,
  FOREIGN KEY (book_id) REFERENCES users(user_id)
      ON UPDATE CASCADE
      ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS returned_checkouts (
  checkout_id UUID PRIMARY KEY,
  book_id UUID NOT NULL,
  user_id UUID NOT NULL,
  checked_out_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3),
  returned_at TIMESTAMP(3) WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP(3)
);
